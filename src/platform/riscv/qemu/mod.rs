pub mod clint;
pub mod uart;

// Some of these constants duplicate constants in the linker script and boot.asm
// and should be changed in both places
pub const PAGE_SIZE: usize = 4096;
pub const HART_COUNT: usize = 4;
pub const HEAP_END: *mut u8 = 0x9000_0000 as *mut u8;
