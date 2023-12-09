use core::panic::PanicInfo;

use crate::riscv;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { riscv::wait_for_interrupt(); }
    }
}
