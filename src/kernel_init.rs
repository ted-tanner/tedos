use crate::riscv;

#[no_mangle]
pub extern "C" fn _kernel_init() -> ! {
    loop {
        unsafe { riscv::wait_for_interrupt(); }
    }
}
