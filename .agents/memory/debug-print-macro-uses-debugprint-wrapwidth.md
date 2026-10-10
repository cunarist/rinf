---
description: The Rust `debug_print!` output is shown with Flutter `debugPrint(rustReport, wrapWidth: 1024)` rather than `print`, so long messages are wrapped instead of truncated by consoles
---

`flutter_package/lib/rinf.dart` used `print(rustReport)`; consoles truncate long single lines. PR #667 (merged 2026-02-18) switched to `debugPrint(rustReport, wrapWidth: 1024)`. The maintainer agreed that `debugPrint` matches the intent of the `debug_print!` macro (debug-only output, throttled). Do not revert to `print` when touching log forwarding.

Evidence: #667
