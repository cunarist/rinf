---
description: Rust-to-Dart streams retain the latest value for newly mounted widgets; generated stream code must keep both broadcast listening and latest-value access
---

Rust-to-Dart streams retain a latest value so newly mounted Dart widgets can
read the most recent signal without waiting for the next emission. When
changing generated stream/controller code, preserve both broadcast/listen
behavior and latest-value initialization semantics.
