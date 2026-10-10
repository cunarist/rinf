---
description: Web errors like `rinf is not defined` or `completeRinfLoad is not a function` were load-order or JS-interop problems (wasm-bindgen, `jsify`), fixed in 7.1.0 and later, not user-code bugs
---

Rinf creates a global `rinf` JS object when the Flutter app loads (666d107d, Feb 2024). Later the generated `hub.js` could run before it existed (`ReferenceError: rinf is not defined`, release builds too, hot restart did not help; the maintainer could not reproduce on macOS). PR #468 / 7.1.0 changed how bindings are created and cleaned up the global namespace, with one revert and re-revert of the module-load change; a contributor PR (#463) also ignored the missing binding. Later, in release builds Dart's `jsify()` stopped producing a usable function (#532, #535), fixed by creating JS functions the new way (#555). The maintainer closed #460 saying `wasm_bindgen` behavior "has changed a little after 0.2.93", so generated `hub.js` referenced a global `rinf` not yet defined. The `Maximum call stack` on web debug with u128 (#444) had an LLVM multivalue theory that was struck out; cause unknown, workaround `rinf wasm --release`, closed when newer Rust no longer reproduced it. Package web code was migrated to `dart:js_interop` separately (#511).

For similar web errors suspect load-order races and toolchain/interop changes first. A sub-path deployment (`--base-href`) needed fixes in both Rinf (#364) and tokio_with_wasm because `hub.js` was loaded from absolute `/pkg` (#298).

Evidence: #298, #364, #444, #460, #463, #468, #532, #535, #555, commits 666d107d, 3d0fb6b8, d9c45a5e; CHANGELOG 7.1.0
