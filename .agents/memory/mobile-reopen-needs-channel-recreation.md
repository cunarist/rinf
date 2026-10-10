---
description: On mobile the process outlives the Flutter UI, so reopening after the back button re-runs Dart init against a live Rust side; channel recreation must work in release, not only on hot restart
---

In 6.x the logic recreating a closed message channel was wrapped in `#[cfg(debug_assertions)]` because it was written for hot restart. On Android, closing with the back button and reopening restarts the Dart isolate but not the native process, so release builds hit the same stale state. Symptoms: hang on the splash screen (#392, a 6.14.0 regression), then after the first fix Dart signals silently stopped arriving (debug did not reproduce).

Fixes: #395 / 4c0c2c4e (infinite parking of the Rust main thread during shutdown, 6.14.1) and #398 / dd56f273 (6.14.2, "Create message channel again if closed", debug-only gate removed). `finalizeRust` on exit request was not the cause. The maintainer could not reproduce one follow-up, so residual cases may remain. Rule: anything "reset on hot restart" must also be safe for mobile reopen in release mode; test relaunch in release. See [[cooperative-shutdown]], [[finalize-rust-api-history]].

Evidence: #390, #392, #395, #398, commits 4c0c2c4e, dd56f273; CHANGELOG 6.14.1, 6.14.2
