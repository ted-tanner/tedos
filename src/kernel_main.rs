use crate::platform::uart::{Uart, UartController};
use crate::platform::{Platform, PlatformPrimitives};
use crate::println;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // TODO: Still need to initialize the CLINT
    Uart::init();

    println!("Hello, world!");

    loop {
        Platform::wait_for_interrupt();
    }
}
