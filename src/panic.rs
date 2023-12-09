use core::panic::PanicInfo;

use crate::{println, riscv};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);

    loop {
        riscv::wait_for_interrupt();
    }
}
