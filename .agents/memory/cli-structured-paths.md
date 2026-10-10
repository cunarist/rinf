---
description: CLI commands must use structured path handling; past fixes covered offline use, nested folders, custom output paths, and `pubspec.yaml` discovery
---

CLI commands have repeatedly been adjusted for offline environments, nested
folders, custom output paths, and platform path behavior. Prefer structured
path handling over string paths, preserve support for configured crates and
nested signal modules, and verify `pubspec.yaml` discovery before changing
command working-directory logic.

See [[windows-path-hazards]]. PR #156 (4.1.4) made the CLI honor `PUB_CACHE` on Windows, since the pub cache is not always under the default user directory.

Evidence: #156
