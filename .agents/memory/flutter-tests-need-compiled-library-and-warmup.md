---
description: Flutter tests with Rinf need `compiledLibPath`, a wait for the first Rust signal, and `tester.runAsync`
---

Widget tests fail to load the library unless `initializeRust(..., compiledLibPath: 'target/release/libhub.dylib')` points at a built library. Rust-to-Dart messages then look missing only because of startup timing: await the first signal (`stream.first`) inside `tester.runAsync`. On Linux, global symbol lookup fails under `flutter test` with `undefined symbol: store_dart_post_cobject`, so `load_os.dart` picks local lookup when the `FLUTTER_TEST` environment variable is set (classes were renamed RustLibraryGlobal/Local). Documented after the reports. See [[native-library-symbol-lookup]].

Evidence: #506, #507, #512, #518, commit 657d78f5
