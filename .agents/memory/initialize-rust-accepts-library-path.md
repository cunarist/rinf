---
description: Custom or embedded targets (flutter-pi, flutter-elinux) load the Rust library through `initializeRust(compiledLibPath: ...)`
---

Rinf could not find libhub.so on flutter-pi because the path was hardcoded; #343 added a configurable compiled library path. Users copy the .so into flutter_assets and pass the absolute path. Dart's load error does not list searched paths. A `GLIBC_2.33 not found` error means the .so was built against a newer libc than the board has. On Windows, a native crate's own DLL dependencies (such as ffmpeg DLLs) must sit next to the exe or hub.dll fails to load even though it exists. The same parameter is used in tests, see [[flutter-tests-need-compiled-library-and-warmup]].

Evidence: #338, #343, #310
