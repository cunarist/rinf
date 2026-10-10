---
description: Windows MSB8066 or 'highest supported language version is 2.18' after moving a project folder is a stale CMake cache; clean everything and reinstall the CLI
---

A user renamed `C:/w` to `C:/working`; the CMake cache kept the old path, giving Dart language-version errors and "cannot resolve build_tool". Fix: `flutter clean`, `cargo clean`, reinstall the CLI, retry from a fresh directory (#570). MSB8066 with no details needs `flutter build windows -v` output; a pub.dev release failed while a git dependency worked in #574, cause unconfirmed, see [[windows-path-hazards]]. MSB8066 is a generic wrapper: arm64 needed a Cargokit target ([[windows-arm64-needs-cargokit-target]]), and multi-line inline tables in `Cargo.toml` also fail ([[cargokit-toml-parser-rejects-multiline-inline-tables]]). The same stale cache appeared when a cloned folder was renamed (#219); `flutter clean` plus `cargo clean` fixed it on any CMake platform. For a new empty project failing on Windows, adding `FLUTTER_EPHEMERAL_DIR` and the `generated_config.cmake` include to `windows/CMakeLists.txt` worked (#657).

Evidence: #219, #570, #574, #657
