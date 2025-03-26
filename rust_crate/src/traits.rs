use std::sync::{MutexGuard, PoisonError};

/// There is no panicking code in the Rinf crate.
/// We assume that any mutex used here will never be poisoned.
/// This trait handles the recovery from a logically poisoned situation,
/// which is unlikely to occur.
pub trait GuardRecovery<'a, T> {
  fn recover(self) -> MutexGuard<'a, T>;
}

impl<'a, T> GuardRecovery<'a, T>
  for Result<MutexGuard<'a, T>, PoisonError<MutexGuard<'a, T>>>
{
  fn recover(self) -> MutexGuard<'a, T> {
    match self {
      Ok(inner) => inner,
      Err(poisoned) => poisoned.into_inner(),
    }
  }
}
