---
description: Windows builds have broken on cross-drive `cd`, missing shell aliases, lost executable bits, and argument escaping; keep scripts path-safe
---

Windows path handling has failed on cross-drive `cd` operations, missing shell
aliases, script executable bits lost after Windows-to-macOS moves, and argument
escaping. Keep Cargokit and script changes path-safe across shells and drives:
use explicit drive-aware shell behavior and quoted structured paths.

See [[cli-structured-paths]].

Two more hazards. Line endings: in #574 MSB8066 appeared with the pub.dev release (LF files) and disappeared with a git dependency (CRLF on Windows). The maintainer noted the LF versus CRLF difference but the root cause was never confirmed, so ask for `flutter build windows -v` first; see [[publish-only-from-linux-ci]] and [[windows-stale-cmake-cache-and-msb8066]]. Path detection: Android Gradle code must not use `PWD`, which exists on macOS but not Windows ([[android-ndk-follows-flutter]]). Also [[windows-gnu-toolchain-breaks-cargo-install]].

Cross-drive `cd` needed `cd /d` in `run_build_tool.cmd` (#295; symptom "Couldn't resolve package build_tool", first seen in #189); upstream had already fixed it, so check upstream Cargokit before patching the vendored copy.

Evidence: #295, #574, #61, commits f474fdb2, c809bb86, 16ae02df
