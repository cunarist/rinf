# Platform Notes

## Web and WASM

Web builds are sensitive because Rinf relies on WASM features that require nightly/std build support plus shared-memory-related flags. Shared memory also requires COOP/COEP headers at runtime; pass them as Flutter web headers during local runs.

The web path crosses JavaScript and generated Dart boundaries. Rinf supports 64-bit integer formats in the Rust/generator layer, but web schemas with 64-bit or wider integer fields should be tested explicitly; use strings for exact IDs, counters, or timestamps when JavaScript number precision would be risky.

Some WASM failures around memory exports have historically been fixed by making runtime memory symbols explicit. If a web build fails after toolchain or wasm-pack changes, inspect generated exports and runtime loader expectations before changing signal code.

In particular, Rust nightly stopped exporting `__heap_base` automatically for wasm targets, and wasm-bindgen thread support needed that symbol exported explicitly. Toolchain fixes may belong in linker flags or the CI install path, not in application signal logic.

## Native Library Loading

Android and ohos use local dynamic-library symbol lookup. Linux tests also switch to local lookup under the test environment. iOS and macOS use framework-style loading, where global native annotations only work when symbols are globally visible.

Endpoint lookup failures usually mean the generated Dart code expects a Rust export that was not produced. Check the signal derive used on the Rust type and regenerate bindings before assuming a platform loader bug.

Windows path handling has failed on cross-drive `cd` operations, missing shell aliases, script executable bits after Windows-to-macOS moves, and argument escaping. Keep Cargokit/script changes path-safe across shells and drives.

## Empty Binary Buffers

Empty byte vectors have historically been unsafe for zero-copy transfer on the Dart boundary. The bridge sends them as null and reconstructs empty buffers on the Dart side. Do not "simplify" this path without testing empty payloads.

## Shutdown and Hot Restart

Native Rust logic should listen for Dart shutdown and exit cooperatively. Hot restart relies on Dart/Rust stopped flags staying synchronized.

`finalizeRust()` matters on native platforms because the Rust thread is otherwise allowed to keep running. On web, finalization is effectively a no-op.
