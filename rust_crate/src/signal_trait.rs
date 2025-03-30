//! This module provides functionality for defining and handling signals
//! in a type-safe way, ensuring that all structs and enums
//! require their inner structs and enums to implement the signal trait.

use crate::channel::SignalReceiver;
use crate::interface::DartSignalPack;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Defines the methods that a type capable of
/// receiving Dart signals must implement.
pub trait DartSignalBinary<T> {
  /// Returns the receiver that listens for signals from Dart.
  ///
  /// If this function is called multiple times,
  /// only the most recent receiver remains active,
  /// and all previous ones become inactive after receiving `None`.
  fn get_dart_signal_receiver() -> SignalReceiver<DartSignalPack<T>>;
}

/// Defines the methods that a type capable of
/// receiving Dart signals must implement.
pub trait DartSignal<T> {
  /// Returns the receiver that listens for signals from Dart.
  /// If this function is called multiple times,
  /// only the most recent receiver remains active,
  /// and all previous ones become inactive after receiving `None`.
  fn get_dart_signal_receiver() -> SignalReceiver<DartSignalPack<T>>;
}

/// Defines the methods that a type capable of
/// sending Rust signals with binary data must implement.
pub trait RustSignalBinary {
  /// Sends a signal to Dart with separate binary data.
  /// Passing data from Dart to Rust is a zero-copy operation.
  fn send_signal_to_dart(&self, binary: Vec<u8>);
}

/// Defines the methods that a type capable of
/// sending Rust signals must implement.
pub trait RustSignal {
  /// Sends a signal to Dart.
  /// Passing data from Dart to Rust is a zero-copy operation.
  fn send_signal_to_dart(&self);
}

/// Ensures that all inner structs and enums
/// within signal structs and enums are also signals.
#[doc(hidden)]
pub trait SignalPiece {}

// Implement the trait for simple primitives.
impl SignalPiece for i8 {}
impl SignalPiece for i16 {}
impl SignalPiece for i32 {}
impl SignalPiece for i64 {}
impl SignalPiece for i128 {}
impl SignalPiece for u8 {}
impl SignalPiece for u16 {}
impl SignalPiece for u32 {}
impl SignalPiece for u64 {}
impl SignalPiece for u128 {}
impl SignalPiece for f32 {}
impl SignalPiece for f64 {}
impl SignalPiece for bool {}
impl SignalPiece for char {}
impl SignalPiece for String {}
impl SignalPiece for &str {}

// Implement the trait for container types.
impl<T> SignalPiece for Box<T> where T: SignalPiece {}
impl<T> SignalPiece for Option<T> where T: SignalPiece {}

// Implement the trait for collection types.
impl<T, const N: usize> SignalPiece for [T; N] where T: SignalPiece {}
impl<T> SignalPiece for Vec<T> where T: SignalPiece {}
impl<T> SignalPiece for HashSet<T> where T: SignalPiece {}
impl<T> SignalPiece for BTreeSet<T> where T: SignalPiece {}
impl<K, V> SignalPiece for HashMap<K, V>
where
  K: SignalPiece,
  V: SignalPiece,
{
}
impl<K, V> SignalPiece for BTreeMap<K, V>
where
  K: SignalPiece,
  V: SignalPiece,
{
}

// Implement the trait for tuples.
impl SignalPiece for () {}
impl<T1> SignalPiece for (T1,) where T1: SignalPiece {}
impl<T1, T2> SignalPiece for (T1, T2)
where
  T1: SignalPiece,
  T2: SignalPiece,
{
}
impl<T1, T2, T3> SignalPiece for (T1, T2, T3)
where
  T1: SignalPiece,
  T2: SignalPiece,
  T3: SignalPiece,
{
}
impl<T1, T2, T3, T4> SignalPiece for (T1, T2, T3, T4)
where
  T1: SignalPiece,
  T2: SignalPiece,
  T3: SignalPiece,
  T4: SignalPiece,
{
}
