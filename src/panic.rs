use core::panic::PanicInfo;

use crate::platform::{Platform, PlatformPrimitives};
use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);
    Platform::abort();
}
