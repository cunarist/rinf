---
description: The recommended pattern is core state in Rust (global OnceLock/OnceCell + Mutex) streamed to Flutter, which only renders; widget state stays in Dart
---

v6 dropped v5-style awaited responses, and users migrating complained that obtaining a reply got harder. The FAQ position: Flutter only displays via the signal stream while state stays in Rust (#292, #294; docs expanded in #301, #333, #419). For a stateful example, keep state in a std `OnceLock` (or `OnceCell`) behind a `Mutex`, answer actions with an empty reply, and stream the latest state to a `StreamBuilder`; the maintainer first suggested lazy_static, then preferred std types (LazyLock suggested in #279). Widget state should not be managed by the backend, only core state. Streams keep the latest value, see [[rust-signal-streams-keep-latest]]; rapid sends need [[streambuilder-skips-rapid-signals]].

Evidence: #230, #231, #279, #292, #294, #301, #333, #419
