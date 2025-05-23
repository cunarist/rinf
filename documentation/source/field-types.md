# Field Types

Rinf uses Bincode serialization under the hood. It currently supports most of the standard types that Bincode does:

- Signed Integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned Integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Floating-Point Numbers: `f32`, `f64`
- Text: `char`, `String`, `&str`
- Boolean: `bool`
- Sequences: `[T; N]`, `Vec<T>`, `HashSet<T>`, `BTreeSet<T>`
- Maps: `HashMap<K, V>`, `BTreeMap<K, V>`
- Standard Library Types: `Option<T>`, `Box<T>`
- Tuple Types: `()` to `(T1, T2, T3, T4)`
- C-style enums
- Enums with inner data

You can nest anything that implements `SignalPiece` inside a `RustSignal` or `DartSignal`. `Serialize` and `Deserialize` also have their own nesting rules. If there are no compile-time errors and the app builds successfully, you're good to go.

To represent abstract data such as time, it’s recommended to use integer-based timestamps or simple primitives.
