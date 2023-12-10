pub mod qemu;
pub mod registers;

pub use registers::*;

use core::arch::asm;

pub fn abort() -> ! {
    unsafe {
        asm!("unimp");
    }

    loop {
        wait_for_interrupt();
    }
}

pub fn wait_for_interrupt() {
    unsafe {
        asm!("wfi");
    }
}
