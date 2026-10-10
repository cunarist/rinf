---
description: Maintainer's cost model for signals: protobuf (de)serialization is the only real UI-blocking factor, Rust-to-Dart binary is zero-copy, and Dart never needs an isolate to call Rust
---

Answering a benchmarking question (#399, no benchmarks exist), the maintainer stated: a signal is a serialized message plus optional raw binary. Dart-to-Rust raw binary needs one memory copy because Rust ownership cannot coexist with garbage-collected Dart objects. Rust-to-Dart raw binary is a zero-copy ownership transfer. Serialization cost is separate and is the only plausible cause of a brief UI freeze (described as unlikely). Dart and Rust (tokio) run on different thread sets and Rust tasks never block the Dart runtime or UI, so wrapping calls in a Dart isolate is unnecessary.

After Dart 3.5 leaf calls with TypedData (#408, implemented in #425) the Dart-to-Rust message copy became avoidable; raw binary from Dart still copies. Send already-serialized or large data as raw binary to skip protobuf cost. Flatbuffers is not supported; use raw binary signals for custom formats (#388).

Evidence: #399, #408, #388
