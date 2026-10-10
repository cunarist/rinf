---
description: Windows arm64 builds failed with MSB8066 until Cargokit gained the `aarch64-pc-windows-msvc` / `windows-arm64` target (#650)
---

Issue #649 (a Windows arm64 CI runner, `flutter build windows --release`) showed the generic MSB8066 custom-build error for `hub.dll.rule`; the reporter then solved it and sent #650, which adds `Target(rust: 'aarch64-pc-windows-msvc', flutter: 'windows-arm64')` to `cargokit/build_tool/lib/src/target.dart`. The author noted that newer upstream Cargokit already had it but they were unsure of the impact of syncing the whole upstream, so only the target was added. The maintainer merged it and promised a release.

MSB8066 is a generic wrapper message, so for arm64 first confirm that the platform's Rust target is listed in Cargokit's target table. See [[windows-stale-cmake-cache-and-msb8066]] and [[cargokit-preserve-upstream-history]].

Evidence: #649, #650
