---
description: Windows builds have broken on cross-drive `cd`, missing shell aliases, lost executable bits, and argument escaping; keep scripts path-safe
---

Windows path handling has failed on cross-drive `cd` operations, missing shell
aliases, script executable bits lost after Windows-to-macOS moves, and argument
escaping. Keep Cargokit and script changes path-safe across shells and drives:
use explicit drive-aware shell behavior and quoted structured paths.

See [[cli-structured-paths]].

Two more hazards. Line endings: a Windows git checkout has CRLF while pub.dev files are LF, which breaks Cargokit's shell scripts and shows up as MSB8066 (#574); see [[publish-only-from-linux-ci]] and [[windows-stale-cmake-cache-and-msb8066]]. Path detection: Android Gradle code must not use `PWD`, which exists on macOS but not Windows ([[android-ndk-follows-flutter]]). Also [[windows-gnu-toolchain-breaks-cargo-install]].

Evidence: #574, #61, commits f474fdb2, c809bb86, 16ae02df
