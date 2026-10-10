---
description: Rinf is a minimal message-passing bridge for app developers, not a general FFI generator; pure-Dart projects, arbitrary function binding, and native-language interop are out of scope
---

Rinf takes the minimal approach: bytes plus a few integers cross the boundary,
and the design aims at app development, not library development. The main
trade-off is serialization, which costs CPU and extra copies for large payloads
such as images; binary signals exist for that case.

Deliberately absent: pure-Dart projects (the FAQ says Flutter GUI apps only),
arbitrary function binding, and Rust-to-native Java/Kotlin/Swift interop (#529,
would need a dedicated maintainer). The bridge engine is Rinf's own; since 5.1.0
no third-party bridge code is vendored, so never copy another project's code in.

Flutter is only the UI layer and business logic lives in Rust on its own thread;
see [[rust-cannot-own-the-main-thread]] and [[state-lives-in-rust]].

Evidence: #172, #218, #262, #529, CHANGELOG 5.1.0
