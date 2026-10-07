---
description: CLI commands must use structured path handling; past fixes covered offline use, nested folders, custom output paths, and `pubspec.yaml` discovery
---

CLI commands have repeatedly been adjusted for offline environments, nested
folders, custom output paths, and platform path behavior. Prefer structured
path handling over string paths, preserve support for configured crates and
nested signal modules, and verify `pubspec.yaml` discovery before changing
command working-directory logic.

See [[windows-path-hazards]].
