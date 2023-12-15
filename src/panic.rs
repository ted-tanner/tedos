use core::panic::PanicInfo;

use crate::platform::{Platform, PlatformPrimitives};
use crate::printbuf::GLOBAL_PRINT_BUF;
use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        GLOBAL_PRINT_BUF.init();
    }

    println!("\nhart {} {}", Platform::curr_hartid(), info);
    Platform::abort();
}
