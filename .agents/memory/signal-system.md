# Signal System

## Type Boundaries

Signal types should stay inside the set that the derive macros and Dart generator can model clearly. Containers such as `Box<T>`, `Option<T>`, arrays, vectors, sets, maps, and small tuples are supported patterns, but generic foreign signal types are rejected.

The generic-type ban is intentional. Dart-side type mapping becomes ambiguous, and failures tend to surface later than Rust compile time. Prefer concrete signal DTOs even if that creates a small amount of duplication.

Recursive signals should use the `Option<Box<T>>` pattern. That shape gives the generator a finite nullable edge while still expressing recursive data.

## Serde Attributes

`#[serde(skip)]` is supported for fields that should be excluded from signal checking and generated Dart output.

Custom serde behavior such as `with`, custom serialize/deserialize functions, `flatten`, and one-sided skip attributes is banned for signal fields. The generator cannot faithfully reproduce those transformations in Dart, so the project catches them at compile time instead of allowing silent wire-format drift.

## Receiver Ownership

Dart-to-Rust receiving is FIFO, but it is not broadcast. A receiver clone becomes the active receiver, and the previous receiver resolves `recv()` to `None`.

Treat `None` from `recv()` as "this receiver was superseded," not as a decoded signal. Calling `get_dart_signal_receiver()` from multiple tasks means only the newest consumer remains live.

## Web-Safe Signal Design

Even when Rust trait support exists for large integer types, web/WASM is a practical boundary. Avoid `i64`-style fields in cross-platform signals; use strings or doubles for IDs, counters, and timestamps that must work on web.
