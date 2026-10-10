---
description: allo-isolate below 0.1.26 crashed during garbage collection on Android 15 (API 35, Pixel 9); Rinf requires 0.1.26 or newer
---

A contributor reported that the example app (or any minimal Rinf app) crashed during garbage collection on the Pixel 9 API 35 emulator and on a real Pixel 9. The root cause was a bug in the `allo-isolate` crate, fixed upstream in 0.1.26 (shekohex/allo-isolate#63). The maintainer fixed it by only bumping `allo-isolate = "0.1.25"` to `"0.1.26"` in `rust_crate/Cargo.toml` (merged 2024-10-22).

Lesson: crashes on brand-new Android releases that appear with no Rinf changes should first be checked against `allo-isolate` releases, since it owns the Rust-to-Dart posting path (see [[store-dart-post-cobject-called-from-rust]]).

Evidence: #464, #466
