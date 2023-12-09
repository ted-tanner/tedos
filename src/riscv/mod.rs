pub mod qemu;
pub mod registers;

pub use registers::*;

use core::arch::asm;

pub unsafe fn wait_for_interrupt() {
    asm!("wfi");
}
