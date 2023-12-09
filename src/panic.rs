use core::panic::PanicInfo;

use crate::{riscv, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    info.location().map(|l| {
        println!(
            "\nkernel panic at {}:{}\n\n{:#?}",
            l.file(),
            l.line(),
            info,
        );
    });

    loop {
        riscv::wait_for_interrupt();
    }
}
