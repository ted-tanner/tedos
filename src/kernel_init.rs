use crate::println;
use crate::riscv;

#[no_mangle]
pub extern "C" fn _kernel_init() -> ! {
    // TODO: Need to do all the bit-fiddling to initialize necessary registers
    riscv::qemu::Uart::init();

    println!("Hello, world!");

    loop {
        riscv::wait_for_interrupt();
    }
}
