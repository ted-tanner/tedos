#![no_std]

mod alloc;
mod kernel_main;
mod panic;
mod platform;

use platform::{Platform, PlatformPrimitives};

#[no_mangle]
unsafe extern "C" fn _kernel_init() {
    Platform::kernel_init();
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        use $crate::platform::uart::{Uart, UartController};

        let _ = write!(Uart::get_ref(), $($args)+);
    })
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n");
    });
    ($fmt:expr) => ({
        $crate::print!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::print!(concat!($fmt, "\n"), $($args)+);
    });
}
