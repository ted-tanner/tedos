use core::slice;
use core::sync::atomic::{AtomicUsize, Ordering};

extern "C" {
    static mut _heap_start: u8; // Defined in the linker script
}

// The kinit heap is used to allocate memory for the kernel before the
// allocator is initialized. Allocated memory cannot be deallocated.
// Once the allocator is initialized, the kinit heap is locked and
// cannot grow anymore. This is because the allocator places its heap
// just after the kinit_heap.
//
// If KINIT_HEAP_POS is 0, the heap is locked (hence it gets
// initialized to 4, one word above 0).
static mut KINIT_HEAP_POS: AtomicUsize = AtomicUsize::new(4);

pub struct KinitHeap;

impl KinitHeap {
    pub unsafe fn alloc<const N: usize>() -> &'static mut [u8; N] {
        let start_pos = KINIT_HEAP_POS.fetch_add(N, Ordering::Relaxed);

        if start_pos == 0 {
            panic!("Tried to access kinit heap while locked");
        }

        let start = (&mut _heap_start as *mut u8).add(start_pos);
        let space = slice::from_raw_parts_mut(start, N);

        space.try_into().unwrap_unchecked()
    }

    pub fn lock() -> usize {
        unsafe { KINIT_HEAP_POS.fetch_and(0, Ordering::Relaxed) }
    }
}
