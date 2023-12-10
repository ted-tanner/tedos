use crate::println;
use crate::riscv;
use crate::riscv::qemu::uart::Uart;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // TODO: Still need to initialize the CLINT
    // Initialize the UART
    Uart::init();

    println!("Hello, world!");

    loop {
        riscv::wait_for_interrupt();
    }
}
