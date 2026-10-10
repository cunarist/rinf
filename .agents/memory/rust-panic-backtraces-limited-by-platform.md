---
description: Panic output in debug mode comes from a custom hook; backtrace quality varies by platform and the effort to improve it was closed as not useful for async apps
---

Rinf 4.6 installed a panic hook (`std::panic::set_hook`) that sends the panic info to Flutter and prints it in the CLI through `debug_print!`, only in debug builds (#177). #179 added backtraces. Issue #180 recorded the limits: on Windows and Android most frames cannot be resolved; on macOS and iOS frames lack file names; Ubuntu looked fine; web panics are handled by wasm_bindgen. Using the default Rust hook or `RUST_BACKTRACE=1/full` made no difference, and disabling the symbol `resolve` step on Android showed more unnamed frames. The maintainer suspected Cargokit or missing `.pdb` files, then closed it because backtraces are not very useful in async apps. Do not spend time chasing full native backtraces; reproduce with a minimal case and use `debug_print!` logs.

Evidence: #177, #179, #180
