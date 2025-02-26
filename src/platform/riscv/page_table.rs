use crate::platform::PageTablePrimitives;

struct Page {
    flags: usize,
    phys_addr: Option<*mut u8>,
}

pub struct RiscVPageTable {
    root: Page,
}

impl PageTablePrimitives for RiscVPageTable {
    unsafe fn init() -> Self {
        unimplemented!()
    }

    fn map(&mut self, virt_addr: usize, phys_addr: usize, flags: usize) {
        unimplemented!()
    }

    fn unmap(&mut self, virt_addr: usize) {
        unimplemented!()
    }

    fn translate(&self, virt_addr: usize) -> Option<usize> {
        unimplemented!()
    }
}
