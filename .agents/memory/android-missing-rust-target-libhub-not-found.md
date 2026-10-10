---
description: Android `libhub.so not found` at runtime is usually a missing rustup Android target, not a Rinf bug
---

A template app failed with a dlopen error for libhub.so although CI passed. Running `rustup target add --toolchain stable x86_64-linux-android` (the emulator ABI) fixed it. Ask for rustc and flutter versions and the installed rustup targets first. For other load failures see [[native-library-symbol-lookup]] and [[android-libcxx-is-users-build-rs]].

Evidence: #483
