#![no_std]

mod alloc;
mod kernel_main;
mod locks;
mod panic;
mod platform;
mod printbuf;

use platform::{Platform, PlatformPrimitives};

#[no_mangle]
unsafe extern "C" fn _kernel_init() {
    Platform::kernel_init();
}
