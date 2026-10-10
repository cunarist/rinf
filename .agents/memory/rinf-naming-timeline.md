---
description: The project was flutter_rust_app_template, then rust_in_flutter (CLI `rifs`), and became Rinf in 4.12.0; old issues use the old names
---

Names in old code, docs, issues and URLs: `flutter_rust_app_template` (before 1.0.0), `rust_in_flutter` / "RIF" / repo `cunarist/rust-in-flutter` (1.0.0 to 4.11.x), and `rinf` from 4.12.0 (2023-10-18). The shorthand crate `rifs` (an alias for `dart run rust_in_flutter ...`) was renamed `rinf` in ba761682.

PR #187 gave the reason: "Rust-In-Flutter is too long for a framework name", and the new name conflicted with no crate or Flutter package. It also moved Flutter code under `flutter_ffi_plugin/` (now `flutter_package/`) and the Rust crate under `rust_crate/`. Old issues quote `RustRequest`, `rifs template`, `RustInFlutter.ensureInitialized`; these map to current Rinf equivalents.

Evidence: PR #187, commit ba761682, CHANGELOG 1.0.0, 3.7.0, 4.12.0
