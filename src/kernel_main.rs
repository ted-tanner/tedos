use crate::platform::uart::{Uart, UartController};
use crate::platform::{Platform, PlatformPrimitives};
use crate::println;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // TODO: Make sure max_harts() is not exceeded
    // TODO: Hart 0 should initialize things. Others should wait_for_interrupt()
    // TODO: Hart 0 shouuld interrupt other harts
    // TODO: Still need to initialize the CLINT
    Uart::init();

    println!("Hello, world!");

    loop {
        Platform::wait_for_interrupt();
    }
}
