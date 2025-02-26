use allocator_api2::alloc::{AllocError, Allocator, GlobalAlloc};

use crate::locks::KMutex;
use crate::platform::{Platform, PlatformPrimitives, PAGE_SIZE};

#[global_allocator]
static PHYS_PAGE_ALLOCATOR: PhysPageAllocator = PhysPageAllocator {
    inner: KMutex::new(PhysPageAllocatorInner {
        free_list_tail: core::ptr::null_mut(),
        unalloced_pages_start: core::ptr::null_mut(),
    }),
};

struct PhysPageAllocatorInner {
    free_list_tail: *mut PhysAllocFreeListBlock,
    unalloced_pages_start: *mut u8,
}

pub struct PhysPageAllocator {
    inner: KMutex<PhysPageAllocatorInner>,
}

// The block must be the size of a single page. The NODES_PER_BLOCK should change if
// other fields in the block are added (or removed).
const NODES_PER_BLOCK: usize = (PAGE_SIZE / core::mem::size_of::<usize>()) - 2;
struct PhysAllocFreeListBlock {
    prev: *mut PhysAllocFreeListBlock,
    count: usize,
    pages: [*mut u8; NODES_PER_BLOCK],
}

impl PhysPageAllocator {
    pub unsafe fn init() {
        const _: () = assert!(core::mem::size_of::<PhysAllocFreeListBlock>() == PAGE_SIZE);

        let first_node = Platform::heap_start() as *mut PhysAllocFreeListBlock;

        *first_node = PhysAllocFreeListBlock {
            prev: core::ptr::null_mut(),
            count: 0,
            pages: [core::ptr::null_mut(); NODES_PER_BLOCK],
        };

        let mut allocator = PHYS_PAGE_ALLOCATOR.inner.lock();

        *allocator = PhysPageAllocatorInner {
            free_list_tail: first_node,
            unalloced_pages_start: Platform::heap_start().add(PAGE_SIZE),
        };
    }

    pub unsafe fn alloc(page_count: usize) -> *mut u8 {
        if page_count == 0 {
            return core::ptr::null_mut();
        }

        let mut allocator = PHYS_PAGE_ALLOCATOR.inner.lock();

        if page_count == 1 {
            let mut curr = allocator.free_list_tail;

            loop {
                if curr.is_null() {
                    // TODO: This is incorrect. curr is the block, not the location we should alloc
                    return allocator.alloc_from_unalloced_region(page_count);
                }

                if (*curr).count != 0 {
                    break;
                }

                curr = (*curr).prev;
            }

            for i in 0..NODES_PER_BLOCK {
                let free_list_page_ref = (*curr).pages.get_unchecked_mut(i);
                if !(*free_list_page_ref).is_null() {
                    let page = *free_list_page_ref;
                    *free_list_page_ref = core::ptr::null_mut();
                    (*curr).count -= 1;
                    return page;
                }
            }

            unreachable!();
        }

        allocator.alloc_from_unalloced_region(page_count)
    }

    pub unsafe fn dealloc(page: *mut u8) {
        debug_assert!(!page.is_null());
        debug_assert!(page >= Platform::heap_start());
        debug_assert!(page < Platform::heap_end());
        debug_assert!(page as usize % PAGE_SIZE == 0);

        let mut allocator = PHYS_PAGE_ALLOCATOR.inner.lock();

        let mut curr = allocator.free_list_tail;
        loop {
            if curr.is_null() {
                let tail = allocator.free_list_tail;
                let new_node = Self::alloc(1) as *mut PhysAllocFreeListBlock;

                *new_node = PhysAllocFreeListBlock {
                    prev: tail,
                    count: 0,
                    pages: [core::ptr::null_mut(); NODES_PER_BLOCK],
                };

                allocator.free_list_tail = new_node;

                curr = new_node;
                break;
            }

            if (*curr).count != NODES_PER_BLOCK {
                break;
            }

            curr = (*curr).prev;
        }

        for i in 0..NODES_PER_BLOCK {
            let free_list_page_ref = (*curr).pages.get_unchecked_mut(i);
            if (*free_list_page_ref).is_null() {
                *free_list_page_ref = page;
                (*curr).count += 1;
                return;
            }
        }
    }
}

impl PhysPageAllocatorInner {
    #[inline]
    unsafe fn alloc_from_unalloced_region(&mut self, page_count: usize) -> *mut u8 {
        let pages_end = self.unalloced_pages_start.add(page_count * PAGE_SIZE);

        if pages_end > Platform::heap_end() {
            panic!("Out of memory");
        }

        let pages_start = self.unalloced_pages_start;
        self.unalloced_pages_start = pages_end;

        pages_start
    }
}

unsafe impl Allocator for PhysPageAllocator {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, AllocError> {
        let page_count = (layout.size() + PAGE_SIZE - 1) / PAGE_SIZE;
        let allocation_start = unsafe { Self::alloc(page_count) };

        if allocation_start.is_null() {
            return Err(AllocError);
        }

        Ok(unsafe {
            core::ptr::NonNull::new_unchecked(core::slice::from_raw_parts_mut(
                allocation_start,
                page_count * PAGE_SIZE,
            ))
        })
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
        Self::dealloc(ptr.as_ptr());
    }

    fn allocate_zeroed(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, AllocError> {
        let allocation = self.allocate(layout)?;

        // Convert from *mut u8 to *mut usize so we can zero out the memory more efficiently (on a 64-bit system,
        // zero 8 bytes at a time instead of 1 byte)
        let allocation_usize = allocation.as_ptr() as *mut usize;
        let allocation_len = allocation.len() / core::mem::size_of::<usize>();
        for i in 0..allocation_len {
            unsafe { allocation_usize.add(i).write(0) };
        }

        // Page size is a multiple of usize, so we don't need to zero out the remaining bytes

        Ok(allocation)
    }

    unsafe fn grow(
        &self,
        ptr: core::ptr::NonNull<u8>,
        _old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, AllocError> {
        Self::dealloc(ptr.as_ptr());
        self.allocate(new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: core::ptr::NonNull<u8>,
        _old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, AllocError> {
        Self::dealloc(ptr.as_ptr());
        self.allocate_zeroed(new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: core::ptr::NonNull<u8>,
        _old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, AllocError> {
        Self::dealloc(ptr.as_ptr());
        self.allocate(new_layout)
    }

    fn by_ref(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

unsafe impl GlobalAlloc for PhysPageAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        PHYS_PAGE_ALLOCATOR.allocate(layout).expect("Alloc should not fail").as_ptr() as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        if !ptr.is_null() {
            PHYS_PAGE_ALLOCATOR.deallocate(core::ptr::NonNull::new_unchecked(ptr), layout);
        }
    }
}
