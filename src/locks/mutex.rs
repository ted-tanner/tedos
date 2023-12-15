use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    lock: AtomicUsize, // usize is the most portable atomic type
}

pub struct MutexGuard<'a, T> {
    data: &'a mut T,
    lock: &'a AtomicUsize,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            lock: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            if self
                .lock
                .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }

            // Use a less aggressive spinlock until the lock appears to be acquireable
            while self.lock.load(Ordering::Relaxed) != 0 {}
        }

        MutexGuard {
            data: unsafe { &mut *self.data.get() },
            lock: &self.lock,
        }
    }

    pub fn attempt_lock(&self) -> Option<MutexGuard<T>> {
        if self
            .lock
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard {
                data: unsafe { &mut *self.data.get() },
                lock: &self.lock,
            })
        } else {
            None
        }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.store(0, Ordering::Release);
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

unsafe impl<T> Sync for Mutex<T> {}
