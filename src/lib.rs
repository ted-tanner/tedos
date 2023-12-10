#![no_std]

mod kernel_init;
mod kernel_main;
mod panic;
mod riscv;

pub use kernel_init::kernel_init;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!($crate::riscv::qemu::uart::Uart::get(), $($args)+);
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
