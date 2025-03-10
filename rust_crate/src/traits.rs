use std::sync::{Mutex, MutexGuard};

pub trait LockAnyway<T> {
    fn lock_anyway(&self) -> MutexGuard<'_, T>;
}

impl<T> LockAnyway<T> for Mutex<T> {
    fn lock_anyway(&self) -> MutexGuard<'_, T> {
        // There is no panicking code in the Rinf crate.
        // We assume that any mutex used here is never poisoned.
        match self.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}
