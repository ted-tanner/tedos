#[cfg(target_arch = "riscv64")]
mod riscv;
#[cfg(target_arch = "riscv64")]
pub type Platform = riscv::RiscVPlatform;

mod hart;

pub trait PlatformPrimitives {
    unsafe fn kernel_init();

    fn page_size() -> usize;

    fn curr_hartid() -> usize;

    fn abort() -> !;
    fn wait_for_interrupt();
}

pub mod uart {
    #[cfg(target_machine = "rv64qemu")]
    pub type Uart = super::riscv::qemu::uart::Uart;

    pub trait UartController: core::fmt::Write {
        fn get_ref() -> Self;

        unsafe fn init();
        fn putchar(byte: u8);
        fn getchar() -> Option<u8>;
    }
}
