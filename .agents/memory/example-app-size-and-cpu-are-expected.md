---
description: Tens of GB in the plugin build folder and high CPU in the example app are expected, not bugs
---

Cargokit keeps per-profile and per-architecture Rust artifacts, so the build folder can reach tens of GB. The example's high CPU is its animated fractal demonstrating multicore parallelism; removing it drops CPU to baseline. Also, a "Finished dev" line comes from the Cargokit `build_tool` crate, not the user's crate; `flutter build --release` does compile Rust in release (verify with `cfg(debug_assertions)`, #59).

Evidence: #246, #209, #59, commit 7a731b55
