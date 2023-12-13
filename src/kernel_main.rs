use core::sync::atomic::{AtomicBool, Ordering};

use crate::platform::uart::{Uart, UartController};
use crate::platform::{Platform, PlatformPrimitives};
use crate::{print, println};

static mut INIT_FINISHED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // TODO: Make sure max_harts() is not exceeded
    // TODO: Hart 0 should initialize things. Others should wait_for_interrupt()
    // TODO: Hart 0 shouuld interrupt other harts
    // TODO: Still need to initialize the CLINT
    if Platform::curr_hartid() == 0 {
        Uart::init();
        println!("TedOS is booting...");
        // TODO
        println!("Boot successful!");
        INIT_FINISHED.store(true, Ordering::SeqCst);
    } else {
        loop {
            Platform::wait_for_interrupt();

            if INIT_FINISHED.load(Ordering::Relaxed) {
                break;
            }
        }

        print!("{}", Platform::curr_hartid());
    }

    loop {
        Platform::wait_for_interrupt();
    }
}
