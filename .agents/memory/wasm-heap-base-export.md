---
description: New Rust nightlies can break web builds (the `__heap_base` export disappeared, #686 open); fixes belong in the linker flags of `webassembly.rs`
---

Rust nightly removed the automatic `__heap_base` wasm export (rust-lang/rust commit b594ef31). wasm-bindgen then fails with "failed to prepare module for threading / failed to find `__heap_base`" (issue #686, still open). PR #680 proposes `-C link-arg=--export=__heap_base` but is not merged.

`rust_crate_cli/src/tool/webassembly.rs` sets `RUSTFLAGS` with the shared-memory, import-memory and max-memory flags plus `__tls_*` and `__wasm_init_tls` exports. These were added in 8.8.1 (c0966c81) after web builds broke on a new nightly. There is no `__heap_base` export in the tree yet. The CLI forces `RUSTUP_TOOLCHAIN=nightly`, so Rinf always follows nightly behavior.

Lesson: when a web build fails after a toolchain change, inspect the exports and linker flags in `webassembly.rs` before touching signal code. #695 raised the Rust floor to 1.91 because wasm-pack 0.15 needs it, see [[rust-floor-1-91-and-runner-pinning]]. Also see [[wasm-shared-memory-headers]].

Evidence: #686, #680, #695, commit c0966c81, CHANGELOG 8.8.1, rust_crate_cli/src/tool/webassembly.rs
