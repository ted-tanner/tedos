use crate::locks::KMutex;
use crate::platform::{Platform, PlatformPrimitives, PAGE_SIZE};

static mut PHYS_PAGE_ALLOCATOR: KMutex<PhysPageAllocator> = KMutex::new(PhysPageAllocator {
    free_list_tail: core::ptr::null_mut(),
    unalloced_pages_start: core::ptr::null_mut(),
});

pub struct PhysPageAllocator {
    free_list_tail: *mut PhysAllocFreeListBlock,
    unalloced_pages_start: *mut u8,
}

const PHYS_ALLOC_BLOCK_SIZE: usize = PAGE_SIZE / core::mem::size_of::<*mut u8>() - 2;

// Must be the size of a single page
#[derive(Debug)]
struct PhysAllocFreeListBlock {
    prev: *mut PhysAllocFreeListBlock,
    count: usize,
    pages: [*mut u8; PHYS_ALLOC_BLOCK_SIZE],
}

impl PhysPageAllocator {
    pub unsafe fn init() {
        let first_node = Platform::heap_start() as *mut PhysAllocFreeListBlock;

        *first_node = PhysAllocFreeListBlock {
            prev: core::ptr::null_mut(),
            count: 0,
            pages: [core::ptr::null_mut(); PHYS_ALLOC_BLOCK_SIZE],
        };

        let mut allocator = PHYS_PAGE_ALLOCATOR.lock();

        *allocator = PhysPageAllocator {
            free_list_tail: first_node,
            unalloced_pages_start: Platform::heap_start().add(PAGE_SIZE),
        };
    }

    pub unsafe fn alloc(page_count: usize) -> *mut u8 {
        if page_count == 0 {
            return core::ptr::null_mut();
        }

        let mut allocator = PHYS_PAGE_ALLOCATOR.lock();

        if page_count == 1 {
            let mut curr = allocator.free_list_tail;

            loop {
                if curr.is_null() {
                    return allocator.alloc_from_unalloced_region(page_count);
                }

                if (*curr).count != 0 {
                    break;
                }

                curr = (*curr).prev;
            }

            for i in 0..PHYS_ALLOC_BLOCK_SIZE {
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

        let mut allocator = PHYS_PAGE_ALLOCATOR.lock();

        let mut curr = allocator.free_list_tail;
        loop {
            if curr.is_null() {
                let tail = allocator.free_list_tail;
                let new_node = Self::alloc(1) as *mut PhysAllocFreeListBlock;

                *new_node = PhysAllocFreeListBlock {
                    prev: tail,
                    count: 0,
                    pages: [core::ptr::null_mut(); PHYS_ALLOC_BLOCK_SIZE],
                };

                allocator.free_list_tail = new_node;

                curr = new_node;
                break;
            }

            if (*curr).count != PHYS_ALLOC_BLOCK_SIZE {
                break;
            }

            curr = (*curr).prev;
        }

        for i in 0..PHYS_ALLOC_BLOCK_SIZE {
            let free_list_page_ref = (*curr).pages.get_unchecked_mut(i);
            if (*free_list_page_ref).is_null() {
                *free_list_page_ref = page;
                (*curr).count += 1;
                return;
            }
        }
    }

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
