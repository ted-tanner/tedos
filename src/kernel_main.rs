use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};

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

        // TODO: Test the allocator
        PhysPageAllocator::init();

        // TODO: Paging

        GLOBAL_PRINT_BUF.init();

        for _ in 0..1 {
            const SIZE: usize = 8000;

            for i in 0..SIZE {
                let alloced = PhysPageAllocator::alloc(1);
            }
        }

        // TODO: PLIC
        // TODO: Filesystem

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
