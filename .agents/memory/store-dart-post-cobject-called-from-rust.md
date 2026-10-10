---
description: The Dart post-object pointer is stored by a Rust-side call because a global exported `store_dart_post_cobject` symbol clashed between Flutter packages using allo-isolate
---

Adding another package depending on allo-isolate on Windows stopped Rust from loading due to duplicate exported symbols. Fix in 7.3.0: Rust calls the store function instead of relying on the shared exported name. This is why exports carry the `rinf` prefix ([[rinf-symbol-prefix]]) and why 7.3.0 users who bumped only `pubspec.yaml` saw nothing load ([[cli-version-skew]]).

Evidence: #520, #521, #522, #543
