---
description: On Windows a GNU default Rust toolchain breaks Rinf CLI install and Cargokit builds; switch to the MSVC host
---

`cargo install` of the CLI failed with `cannot find -lktmw32` because the default toolchain was windows-gnu. Fix: `rustup default stable-x86_64-pc-windows-msvc`. Cargokit reads `default_host_triple` rather than the default toolchain, so setting it to msvc in rustup's `settings.toml` also fixes builds.

Evidence: #491
