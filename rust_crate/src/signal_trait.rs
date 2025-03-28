//! This module provides functionality for defining and handling signals
//! in a type-safe way, ensuring that all structs require their inner
//! structs to implement the signal trait.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// Ensures that structs with derives macros from Rinf
/// enforce all inner structs to be included in code generation.
#[doc(hidden)]
pub trait ForeignSignal {}

// Implement the trait for simple primitives.
impl ForeignSignal for () {}
impl ForeignSignal for i8 {}
impl ForeignSignal for i16 {}
impl ForeignSignal for i32 {}
impl ForeignSignal for i64 {}
impl ForeignSignal for i128 {}
impl ForeignSignal for u8 {}
impl ForeignSignal for u16 {}
impl ForeignSignal for u32 {}
impl ForeignSignal for u64 {}
impl ForeignSignal for u128 {}
impl ForeignSignal for f32 {}
impl ForeignSignal for f64 {}
impl ForeignSignal for bool {}
impl ForeignSignal for char {}
impl ForeignSignal for String {}
impl ForeignSignal for &str {}

// Implement the trait for collection types.
impl<T> ForeignSignal for Option<T> where T: ForeignSignal {}
impl<T> ForeignSignal for Box<T> where T: ForeignSignal {}
impl<T> ForeignSignal for Vec<T> where T: ForeignSignal {}
impl<T> ForeignSignal for HashSet<T> where T: ForeignSignal + Eq + Hash {}
impl<T> ForeignSignal for BTreeSet<T> where T: ForeignSignal {}
impl<K, V> ForeignSignal for HashMap<K, V>
where
  K: ForeignSignal + Eq + Hash,
  V: ForeignSignal,
{
}
impl<K, V> ForeignSignal for BTreeMap<K, V>
where
  K: ForeignSignal,
  V: ForeignSignal,
{
}

// Implement the trait for tuples.
impl<T1> ForeignSignal for (T1,) where T1: ForeignSignal {}
impl<T1, T2> ForeignSignal for (T1, T2)
where
  T1: ForeignSignal,
  T2: ForeignSignal,
{
}
impl<T1, T2, T3> ForeignSignal for (T1, T2, T3)
where
  T1: ForeignSignal,
  T2: ForeignSignal,
  T3: ForeignSignal,
{
}
