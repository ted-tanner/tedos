use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::Ordering;

use crate::platform::uart::{Uart, UartController};
use crate::platform::{Platform, PlatformPrimitives};
use crate::printbuf::GLOBAL_PRINT_BUF;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let printbuf_prev_state = unsafe {
        // This will return immediately if the UART has already been
        // initialized, so no issue here
        Uart::init();
        // Mark the print buffer as in-use to prevent any other harts from
        // printing. This is just a best-effort attempt. It's possible that
        // another hart initializes the print buffer and marks it as not
        // in-use. While undesirable, this is outcome is not catastrophic;
        // output will just be jarbled.
        GLOBAL_PRINT_BUF.in_use.swap(true, Ordering::Acquire)
    };

    // There isn't much we can do if this fails, so just ignore the error
    let _ = writeln!(
        Uart::get_ref(),
        "\nhart {} {}",
        Platform::curr_hartid(),
        info
    );

    unsafe {
        // Restore the print buffer's state. It may have been initialized during this time
        // so the state may have changed. This can be safely ignored because we are just
        // making a best-effort attempt to avoid jarbled output from race conditions.
        let _ = GLOBAL_PRINT_BUF.in_use.compare_exchange(
            true,
            printbuf_prev_state,
            Ordering::Release,
            Ordering::Relaxed,
        );
    }

    Platform::abort();
}
