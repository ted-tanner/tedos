// TODO: Disable interrupts while holding a lock (take care they
//       don't get re-enabled)
//           - Need a per-CPU data structure to count times interrupts
//             were disabled (and re-enable them when the count drops
//             to zero)
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct KMutex<T> {
    data: UnsafeCell<T>,
    is_locked: AtomicBool,
}

pub struct KMutexGuard<'a, T> {
    data: &'a mut T,
    is_locked: &'a AtomicBool,
}

impl<T> KMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> KMutexGuard<T> {
        loop {
            if self
                .is_locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }

            // Use a less aggressive spinlock until the lock appears to be acquireable
            while self.is_locked.load(Ordering::Relaxed) {
                core::hint::spin_loop()
            }
        }

        KMutexGuard {
            data: unsafe { &mut *self.data.get() },
            is_locked: &self.is_locked,
        }
    }

    pub fn attempt_lock(&self) -> Option<KMutexGuard<T>> {
        if self
            .is_locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(KMutexGuard {
                data: unsafe { &mut *self.data.get() },
                is_locked: &self.is_locked,
            })
        } else {
            None
        }
    }
}

impl<T> Drop for KMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.is_locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for KMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<T> DerefMut for KMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

unsafe impl<T> Sync for KMutex<T> {}
