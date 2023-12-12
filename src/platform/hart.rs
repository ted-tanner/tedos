use core::mem;

use crate::platform::{Platform, PlatformPrimitives};

#[repr(transparent)]
struct HartMetadata {
    scratch: [usize; 5],
}

extern "C" {
    // Defined in the linker script
    static mut _hartlist_start: HartMetadata;
    static mut _hartlist_end: HartMetadata;
}

pub fn max_harts() -> usize {
    unsafe {
        (addr(&mut _hartlist_end) - addr(&mut _hartlist_start)) / mem::size_of::<HartMetadata>()
    }
}

pub fn scratch_ptr() -> *mut usize {
    let hartid = Platform::curr_hartid();

    unsafe {
        let metadata = ((&mut _hartlist_start) as *mut HartMetadata).add(hartid);
        (*metadata).scratch.as_mut_ptr()
    }
}

#[inline(always)]
fn addr(ptr: &mut HartMetadata) -> usize {
    ptr as *mut HartMetadata as usize
}
