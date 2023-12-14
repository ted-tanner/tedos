use core::panic::PanicInfo;

use crate::platform::{Platform, PlatformPrimitives};
use crate::{printbuf, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        printbuf::get_ref().init();
    }
    println!("\n{}", info);
    Platform::abort();
}
