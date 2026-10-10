---
description: Permission-denied or wrong current directory from Rust file access (Android os error 13, macOS sandbox cwd) is OS app-permission behavior, not a Rinf bug; the maintainer closes these and points to discussions
---

Reports: `fs::read_dir("./")` on Android panicked with os error 13 even after storage permissions and `requestLegacyExternalStorage` (#351); on macOS from VSCode `current_dir` was the app container `~/Library/Containers/<id>/Data`, not `native/hub`, unlike `cargo test` (#321); reading a directory returned permission denied on iOS/macOS until app entitlements were set (#347). The maintainer closed these as app-permission issues (Android permissions guidance, discussions #322, #175, #348). The maintainer's advice was to set the Flutter app's platform permissions; for sandboxed platforms see [[file-access-from-rust-needs-app-sandbox-paths]]. The Android user reported the app stopped responding after the panic.

Evidence: #321, #347, #351, #133
