#![no_std]

mod kernel_init;
mod panic;
mod riscv;

pub use kernel_init::_kernel_init;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!($crate::riscv::qemu::Uart::get(), $($args)+);
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
