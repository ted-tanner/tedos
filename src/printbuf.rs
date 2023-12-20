use core::sync::atomic::{AtomicBool, Ordering};

use crate::alloc::KinitHeap;
use crate::platform::uart::{Uart, UartController};

const PRINT_BUF_SIZE: usize = 4096;

pub static mut GLOBAL_PRINT_BUF: PrintBuf = PrintBuf {
    buf: 0 as *mut _,
    pos: 0,

    // Mark as true until initialized
    in_use: AtomicBool::new(true),
};

pub struct PrintBuf {
    buf: *mut u8,
    pos: usize,

    pub in_use: AtomicBool,
}

impl PrintBuf {
    pub unsafe fn init(&mut self) {
        self.buf = KinitHeap::alloc::<PRINT_BUF_SIZE>() as *mut _;
        self.in_use.store(false, Ordering::Release);
    }

    pub unsafe fn flush(&mut self) {
        let end = self.buf.add(self.pos);
        let mut pos = self.buf;

        while pos < end {
            Uart::putchar(*pos);
            pos = pos.add(1);
        }

        self.pos = 0;
    }

    pub unsafe fn push(&mut self, bytes: &[u8]) {
        let buf_space = PRINT_BUF_SIZE - self.pos;

        if buf_space >= bytes.len() {
            let mut should_flush = false;

            for b in bytes {
                *self.buf.add(self.pos) = *b;
                self.pos += 1;

                if *b == b'\n' {
                    should_flush = true;
                }
            }

            if should_flush {
                self.flush();
            }

            return;
        }

        let mut bytes_pos = bytes.as_ptr();
        for _ in 0..buf_space {
            *self.buf.add(self.pos) = *bytes_pos;

            self.pos += 1;
            bytes_pos = bytes_pos.add(1);
        }

        self.flush();

        // buf_space >= bytes.len() evaluated to false, so we know we
        // have more bytes to write
        let mut chunks = bytes[buf_space..].chunks(PRINT_BUF_SIZE);
        let last_chunk = chunks.next_back().unwrap_unchecked();

        for chunk in chunks {
            for b in chunk {
                *self.buf.add(self.pos) = *b;
                self.pos += 1;
                // No need to check for '\n' because we will need to
                // flush anyway
            }

            self.flush();
        }

        let mut should_flush = false;
        for b in last_chunk {
            *self.buf.add(self.pos) = *b;
            self.pos += 1;

            if *b == b'\n' {
                should_flush = true;
            }
        }

        if should_flush {
            self.flush();
        }
    }
}

impl core::fmt::Write for PrintBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            self.push(s.as_bytes());
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        use core::sync::atomic::Ordering;

        let print_buf = unsafe { &mut $crate::printbuf::GLOBAL_PRINT_BUF };

        loop {
            if print_buf
                .in_use
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }

            // Use a less aggressive spinlock until the lock appears to be acquireable
            while print_buf.in_use.load(Ordering::Relaxed) {}
        }

        let _ = write!(print_buf, $($args)+);

        print_buf.in_use.store(false, Ordering::Release);
    })
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n");
    });
    ($fmt:expr) => ({
        $crate::print!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::print!(concat!($fmt, "\n"), $($args)+);
    });
}
