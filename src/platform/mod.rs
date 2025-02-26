#[cfg(target_arch = "riscv64")]
mod riscv;
#[cfg(target_arch = "riscv64")]
pub type Platform = riscv::RiscVPlatform;
#[cfg(target_arch = "riscv64")]
pub type PageTable = riscv::page_table::RiscVPageTable;

// #[cfg(target_arch = "aarch64")]
// mod aarch64;

#[cfg(target_arch = "riscv64")]
pub const PAGE_SIZE: usize = 4096;

pub trait PlatformPrimitives {
    unsafe fn kernel_init();

    fn hart_count() -> usize;
    fn heap_start() -> *mut u8;
    fn heap_end() -> *mut u8;

    fn curr_hartid() -> usize;

    fn enable_interrupts();
    fn disable_interrupts();

    fn abort() -> !;
    fn wait_for_interrupt();
}

pub trait PageTablePrimitives {
    unsafe fn init() -> Self;

    fn map(&mut self, virt_addr: usize, phys_addr: usize, flags: usize);
    fn unmap(&mut self, virt_addr: usize);
    fn translate(&self, virt_addr: usize) -> Option<usize>;
}

pub mod uart {
    #[cfg(target_machine = "rv64qemu")]
    pub type Uart = super::riscv::qemu::uart::Uart;

    pub trait UartController {
        // No panics allowed in init() or putchar()
        fn get_ref() -> &'static mut Self;

        unsafe fn init();
        fn putchar(byte: u8);
        fn getchar() -> Option<u8>;
    }
}
