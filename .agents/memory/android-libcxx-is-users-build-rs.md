---
description: Android load failures from crates needing C++ (rodio, surrealdb) are fixed by a user `build.rs` linking `c++_shared`; Rinf deliberately does not link libc++
---

A crate with C++ dependencies works on Linux, Windows, macOS, iOS and web but fails on Android at initialization with `libc++_shared.so` not found or unresolved symbols like `__gxx_personality_v0`. The cause is the crate, not Rinf (rodio issue 404).

Linking libc++ by default was rejected: duplicate native symbols across Flutter plugins are risky, especially with static linking on Apple (the allo-isolate symbol clash was the precedent, see [[rinf-symbol-prefix]]). The `link-cplusplus` crate does not help. The FAQ documents the fix: add a `build.rs` to the hub crate emitting `cargo:rustc-link-lib=c++_shared` for Android (the FAQ snippet also links `stdc++`). Alternatives: gate the dependency with `[target.'cfg(not(target_os = "android"))'.dependencies]`, or set `-DANDROID_STL=c++_shared`.

Triage pattern from the 43-comment thread: reproduce with the clean template, then bisect by adding dependencies one at a time. A similar "works under cargo build, undefined symbols under flutter run" class hit gstreamer and slint on macOS, see [[macos-c-dependent-crates-link-errors]].

Evidence: #280, #210, documentation/source/frequently-asked-questions.md
