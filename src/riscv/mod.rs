pub mod qemu;
pub mod registers;

pub use registers::*;

use core::arch::asm;

pub fn wait_for_interrupt() {
    unsafe {
        asm!("wfi");
    }
}
