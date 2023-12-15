use core::sync::atomic::{AtomicU8, Ordering};

use crate::platform::{Platform, PlatformPrimitives};
use crate::printbuf::GLOBAL_PRINT_BUF;
use crate::println;

// 0 = uninitialized, 1 = initializing, 2 = initialized
static mut PRINT_BUFFER_INIT_STATE: AtomicU8 = AtomicU8::new(0);
static mut PAGING_AND_ALLOC_INIT_STATE: AtomicU8 = AtomicU8::new(0);
static mut PLIC_INIT_STATE: AtomicU8 = AtomicU8::new(0);
static mut FILESYSTEM_INIT_STATE: AtomicU8 = AtomicU8::new(0);

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // TODO: Make sure max_harts() is not exceeded
    // TODO: Hart 0 should initialize things. Others should wait_for_interrupt()
    // TODO: Hart 0 shouuld interrupt other harts
    // TODO: Still need to initialize the CLINT

    let state = PRINT_BUFFER_INIT_STATE
        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(1);
    if state == 0 {
        GLOBAL_PRINT_BUF.init();
        println!(
            "UART and print buffer initialized on hart {}",
            Platform::curr_hartid()
        );
        // TODO
        PRINT_BUFFER_INIT_STATE.store(2, Ordering::Release);
    }

    while PRINT_BUFFER_INIT_STATE.load(Ordering::Relaxed) < 2 {}

    let state = PAGING_AND_ALLOC_INIT_STATE
        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(1);
    if state == 0 {
        // TODO
        println!(
            "Initializing paging subsystem and kernel allocator on hart {}",
            Platform::curr_hartid()
        );
        // TODO: Print buffer init relies upon the kinitheap, so can't lock it until after print buffer init
        PAGING_AND_ALLOC_INIT_STATE.store(2, Ordering::Release);
    }

    let state = PLIC_INIT_STATE
        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(1);
    if state == 0 {
        // TODO
        println!("Initializing PLIC on hart {}", Platform::curr_hartid());
        PLIC_INIT_STATE.store(2, Ordering::Release);
    }

    let state = FILESYSTEM_INIT_STATE
        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(1);
    if state == 0 {
        // TODO
        println!(
            "Initializing filesystem on hart {}",
            Platform::curr_hartid()
        );
        FILESYSTEM_INIT_STATE.store(2, Ordering::Release);
    }

    while PAGING_AND_ALLOC_INIT_STATE.load(Ordering::Relaxed) < 2
        || PLIC_INIT_STATE.load(Ordering::Relaxed) < 2
        || FILESYSTEM_INIT_STATE.load(Ordering::Relaxed) < 2
    {
        Platform::wait_for_interrupt();
    }

    if Platform::curr_hartid() == 0 {
        println!("TedOS booted successfully!");
    }

    loop {
        Platform::wait_for_interrupt();
    }
}
