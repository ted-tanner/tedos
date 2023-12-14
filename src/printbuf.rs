use core::sync::atomic::{AtomicU8, Ordering};

use crate::alloc::KinitHeap;
use crate::platform::uart::{Uart, UartController};

const PRINT_BUF_SIZE: usize = 2048;

static mut BUF: PrintBuf = PrintBuf {
    buf: 0 as *mut _,
    pos: 0,

    // 0 = uninitialized, 1 = initializing, 2 = ready, 3 = in use
    state: AtomicU8::new(0),
};

#[inline(always)]
pub fn get_ref() -> &'static mut PrintBuf {
    unsafe { &mut BUF }
}

pub struct PrintBuf {
    buf: *mut u8,
    pos: usize,

    pub state: AtomicU8,
}

impl PrintBuf {
    pub unsafe fn init(&mut self) {
        let state = self
            .state
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::Relaxed)
            .unwrap_or(1);

        if state != 0 {
            while self.state.load(Ordering::Relaxed) < 2 {}
            return;
        }

        unsafe {
            Uart::init();
        }
        self.buf = KinitHeap::alloc::<PRINT_BUF_SIZE>() as *mut _;

        self.state.store(2, Ordering::Relaxed);
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

        let print_buf = $crate::printbuf::get_ref();

        while print_buf
            .state
            .compare_exchange(2, 3, Ordering::SeqCst, Ordering::Relaxed)
            .is_err()
        {}

        let _ = write!(print_buf, $($args)+);

        print_buf.state.store(2, Ordering::Relaxed);
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
