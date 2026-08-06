# Platform Builds

Use this skill when a platform build, load, or lifecycle path fails.

## Triage Order

First identify whether the failure is build-time, dynamic-library loading, generated-symbol lookup, or runtime lifecycle shutdown. Platform errors often look similar until separated this way.

## Android and ohos

These platforms use local dynamic-library symbols. If endpoint lookup fails, verify the Rust signal derive and regenerate bindings before changing loader logic.

For Gradle or SDK execution errors, check wrapper/tool versions and platform SDK availability before assuming a Rust bridge problem.

## iOS and macOS

After adding Rust dependencies or changing native build integration, refresh native package metadata and rebuild the app. Framework loading depends on expected framework layout and symbol visibility.

Global native annotations only work for globally visible symbols; endpoint lookup may still need dynamic lookup behavior.

## Linux

Linux app runs and tests can use different symbol-loading paths. Test environments may force local symbol lookup, so a failure in tests does not always reproduce the normal app loader path.

If the library cannot be found, verify the actual built artifact path and use the explicit compiled-library override to isolate path issues from symbol issues.

## Windows

Windows ARM64 support came from targeted external work. When touching platform lists, Cargokit, or native loading, keep ARM64 assumptions in review even if you cannot build it locally.

Windows build scripts have failed on paths from different drives and on argument escaping. Prefer explicit drive-aware shell behavior and quoted structured paths.

When upgrading Rust toolchain versions, Windows CI may fail due to CMake cache conflicts with Visual Studio generator versions. The fix requires comprehensive cleanup of CMake cache files and explicit generator specification:

- Clean `CMakeCache.txt`, `CMakeFiles/`, `*.cmake`, and `cmake_install.cmake` in the build directory
- Set `CMAKE_GENERATOR="Visual Studio 17 2022"` environment variable before building
- Use `-ErrorAction SilentlyContinue` on PowerShell Remove-Item commands to handle missing files gracefully

This pattern should be applied to all Windows build steps in CI workflows.

## eLinux

eLinux support exists as a platform surface. When editing package platform lists, generated plugin metadata, or Cargokit target handling, preserve eLinux entries unless intentionally removing support.

## Web and WASM

Web builds are the most toolchain-sensitive path. Confirm the wasm target, nightly/std build requirements, wasm-pack behavior, and shared-memory flags before changing application code.

Shared-memory WASM requires COOP/COEP headers. Use the project server helper or pass equivalent Flutter web headers during local runs.

64-bit or wider integer signal fields need explicit web testing. Replace them with strings when exact precision matters across the JavaScript boundary.

Recent wasm failures were fixed by installing wasm-pack with the right toolchain and explicitly exporting `__heap_base` for wasm-bindgen thread preparation. Check linker/toolchain behavior before changing bridge semantics.

## Shutdown and Hot Restart

Native apps should call finalization when lifecycle control matters. Rust tasks should listen for Dart shutdown and exit cooperatively. Web finalization is effectively a no-op, so do not use web behavior as proof that native shutdown is correct.
