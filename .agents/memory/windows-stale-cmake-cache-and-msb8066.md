---
description: Windows MSB8066 or 'highest supported language version is 2.18' after moving a project folder is a stale CMake cache; clean everything and reinstall the CLI
---

A user renamed `C:/w` to `C:/working`; the CMake cache kept the old path, giving Dart language-version errors and "cannot resolve build_tool". Fix: `flutter clean`, `cargo clean`, reinstall the CLI, retry from a fresh directory (#570). MSB8066 with no details needs `flutter build windows -v` output; CRLF line endings from a git checkout are another cause, see [[windows-path-hazards]] (#574). For a new empty project failing on Windows, adding `FLUTTER_EPHEMERAL_DIR` and the `generated_config.cmake` include to `windows/CMakeLists.txt` worked (#657).

Evidence: #570, #574, #657
