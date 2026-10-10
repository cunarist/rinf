---
description: Dart 3.5 / Flutter 3.24 is a deliberate floor: Dart-to-Rust sends use FFI leaf calls with zero-copy TypedData, introduced in Rinf 7.0
---

Before 7.0 the Dart floor was 3.0.5. Commit 835c6415 (Sep 2024) rewrote the FFI layer to `@Native(isLeaf: true)` functions that receive pointers into Dart `TypedData` instead of `malloc` plus copy, and 7.0.0 raised the floor to Dart 3.5 (#408, #425). Do not lower it by reintroducing copy-based calls and do not raise it incidentally ([[dependabot-limited-to-github-actions]]). Web headers: [[web-headers-via-flutter-args-not-sdk-patching]].

Performance answers given by the maintainer: no benchmarks exist; copying Dart bytes into Rust is inherently needed (Rust ownership cannot coexist with GC objects) and serialization is the real UI-blocking cost. For large or serialization-free payloads (images, video frames) use the binary signal variant; bytes were chosen early so any serializable type works and big buffers cross without copies (#40). Binary signals exist because element-wise typed collections hit size limits: in the Protobuf era a repeated float field over 64 MiB failed to decode in Dart, so 4.0.0 added a separate blob field (#144, #149), see [[empty-buffers-sent-as-null]] for its sibling bug. Switching to flatbuffers was not an option (#374).

Evidence: #408, #425, #399, #388, #374, #40, commits 835c6415, fb082e9e; CHANGELOG 7.0.0
