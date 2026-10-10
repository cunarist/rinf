---
description: On macOS, crates wrapping C libraries (gstreamer, slint, machineid-rs) failed with undefined symbols under `flutter run` only; system headers go into the Xcode project
---

Undefined-symbol link errors appeared only through `flutter run`, not `cargo build`. Tried without success: `-force_load`, `-all_load`, forcing fat or thin lipo output (the fat-vs-thin difference was shown not to be the cause). The common factor was non-pure-Rust dependencies. Adding a system header to the macos folder fixed machineid-rs (4.5.0). For non-system C libraries the user may need to add headers via the `.xcodeproj` or use the `cc` crate in `build.rs`.

Some reporters on M1 Macs also hit "libhub.a not found" that CI missed because `macos-latest` was Intel then; running `pod install` in the platform folder was a reported workaround. See [[android-libcxx-is-users-build-rs]] for the Android analogue.

Evidence: #83 (29 comments), #110, commit 42f1e0c5
