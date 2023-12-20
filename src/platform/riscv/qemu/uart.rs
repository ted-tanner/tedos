use core::sync::atomic::{AtomicU8, Ordering};

use crate::platform::uart::UartController;

const UART_BASE: *mut u8 = 0x1000_0000 as *mut u8;

const UART_IER: *mut u8 = 0x1000_0001 as *mut u8;
const UART_FCR: *mut u8 = 0x1000_0002 as *mut u8;
const UART_LCR: *mut u8 = 0x1000_0003 as *mut u8;
const UART_LSR: *mut u8 = 0x1000_0005 as *mut u8;

const UART_DLL: *mut u8 = UART_BASE;
const UART_DLM: *mut u8 = UART_IER;

// 0 = uninitialized, 1 = initializing, 2 = initialized
static mut INIT_STATE: AtomicU8 = AtomicU8::new(0);

static mut UART: Uart = Uart;

pub struct Uart;

impl UartController for Uart {
    fn get_ref() -> &'static mut Self {
        unsafe { &mut UART }
    }

    unsafe fn init() {
        if INIT_STATE
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Is already initializing or initialized
            return;
        }

        // Set word length to 8 bits
        let lcr_value = 0x03;
        UART_LCR.write_volatile(lcr_value);

        // Enable FIFO
        UART_FCR.write_volatile(0x01);

        // Enable receive interrupts
        UART_IER.write_volatile(0x01);

        // Set baud rate to 2400 (from NS16550A specification)
        const CLOCK_RATE: f64 = 22_729_000.0; // 22.729 MHz
        const BAUD_RATE: f64 = 2_400.0;
        // Add 1.0 to the divisor to round up (equivalent to ceil() when truncating to an int)
        const DIVISOR: u16 = ((CLOCK_RATE / (BAUD_RATE * 16.0)) + 1.0) as u16;
        const DIVISOR_LEAST: u8 = (DIVISOR & 0xff) as u8;
        const DIVISOR_MOST: u8 = (DIVISOR >> 8) as u8;

        // Set DLAB to 1 (DLAB is bit 7 of LCR)
        UART_LCR.write_volatile(lcr_value | (1 << 7));

        // Set DLL and DLM
        UART_DLL.write_volatile(DIVISOR_LEAST);
        UART_DLM.write_volatile(DIVISOR_MOST);

        // Set DLAB to 0
        UART_LCR.write_volatile(lcr_value);

        INIT_STATE.store(2, Ordering::Release);
    }

    fn getchar() -> Option<u8> {
        unsafe {
            if UART_LSR.read_volatile() & 0b01 == 0 {
                // Nothing available to read
                return None;
            }

            Some(UART_BASE.read_volatile())
        }
    }

    fn putchar(c: u8) {
        unsafe {
            UART_BASE.write_volatile(c);
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            Uart::putchar(c);
        }

        Ok(())
    }
}
