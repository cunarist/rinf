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

Linux `flutter test` is the case needing local lookup: global (RTLD_GLOBAL) lookup fails there with `undefined symbol: store_dart_post_cobject`, so `load_os.dart` picks local lookup when `FLUTTER_TEST` is set. Android always uses local lookup (dart-lang/native#923). See [[flutter-tests-need-compiled-library-and-warmup]], [[android-missing-rust-target-libhub-not-found]].

Evidence: #512, #518, commit 657d78f5
