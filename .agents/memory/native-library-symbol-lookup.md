---
description: Android, ohos, and Linux tests use local symbol lookup while Apple uses framework loading; endpoint lookup failures usually mean a missing Rust export
---

Android and ohos use local dynamic-library symbol lookup. Linux tests also
switch to local lookup under the test environment, so a failure in tests does
not always reproduce the normal app loader path. iOS and macOS use
framework-style loading, where global native annotations only work when symbols
are globally visible.

Endpoint lookup failures usually mean the generated Dart code expects a Rust
export that was not produced. Check the signal derive used on the Rust type and
regenerate bindings before assuming a platform loader bug.
