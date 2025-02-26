use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};

use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;

use crate::alloc::PhysPageAllocator;
use crate::platform::uart::{Uart, UartController};
use crate::platform::{Platform, PlatformPrimitives};
use crate::printbuf::GLOBAL_PRINT_BUF;
use crate::println;

static mut INIT_DONE: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // TODO: Make sure max_harts() is not exceeded
    // TODO: Hart 0 should initialize things. Others should wait_for_interrupt()
    // TODO: Hart 0 shouuld interrupt other harts
    // TODO: Still need to initialize the CLINT

    if Platform::curr_hartid() == 0 {
        Uart::init();
        let _ = writeln!(Uart::get_ref(), "Booting...");

        PhysPageAllocator::init();

        // TODO: Paging

        GLOBAL_PRINT_BUF.init();

        // TODO: PLIC
        // TODO: Filesystem

        {
            let mut test_vec = Vec::new();
            for i in 0..30000 {
                test_vec.push(i);
            }

            let _ = println!("len: {}, capacity: {}", test_vec.len(), test_vec.capacity());
        }

        println!("TedOS booted successfully!");
        INIT_DONE.store(true, Ordering::Release);
    } else {
        while !INIT_DONE.load(Ordering::Relaxed) {
            Platform::wait_for_interrupt();
        }
    }

    println!("Hart {} is online", Platform::curr_hartid());

    loop {
        Platform::wait_for_interrupt();
    }
}
