---
description: Rust minimum is 1.91 (it was 1.88 until #695) because wasm-pack 0.15 needs it; CI runners are pinned because windows-latest moved to VS 2026
---

wasm-pack 0.15 needs Rust 1.91 (via cargo-platform 0.3.3). Instead of building the web tools with a separate nightly to keep the 1.88 floor, #695 raised (after the closed #688) `rust-version`, CI and docs to 1.91. The floor is now Rust 1.91 (`rust_crate/Cargo.toml`) and Dart 3.5 (`flutter_package/pubspec.yaml`).

Runners were pinned in the same change (ubuntu-24.04, windows-2022, macos-26) because windows-latest became windows-2025 with only Visual Studio 2026, while the supported Flutter versions need VS 2019/2022. See [[pin-exact-versions]].

Do not change floors incidentally, see [[dependabot-limited-to-github-actions]]. Related web breakage: [[wasm-heap-base-export]].

Evidence: #688, #695, #686, CHANGELOG 8.9.0
