---
description: Rust nightly stopped exporting `__heap_base` for wasm, so wasm-bindgen threads need it exported explicitly; fix toolchain, not signal code
---

Some WASM failures around memory exports have historically been fixed by
making runtime memory symbols explicit. In particular, Rust nightly stopped
exporting `__heap_base` automatically for wasm targets, and wasm-bindgen thread
support needed that symbol exported explicitly. Installing wasm-pack with the
right toolchain was part of the same fix.

If a web build fails after toolchain or wasm-pack changes, inspect generated
exports and runtime loader expectations before changing signal code. Fixes
usually belong in linker flags or the CI install path, not in application
signal logic.

See [[wasm-shared-memory-headers]].
