---
description: The Android plugin's sdkVersion must stay overridable by the parent app, and Gradle snippets must tolerate missing properties
---

NDK 26 rejected minSdk 16 (CXX1110). The fix PR let the app override the default, but CI caught a Groovy `Math.max(null, null, 31)` failure when properties were missing, so the patch was reworked. Keep the Android template config in line with the latest Flutter defaults (#358).

The example app's stale Android and Linux config that broke CI in early 2025 was fixed by deleting `flutter_package/example/android` and rerunning `flutter create . --org com.cunarist --project-name example_app`, not by hand-migrating Gradle; with JDK 21 the wrapper needed Gradle 8.4. A PR fixing CI was merged despite failing checks (#508, #510).

Evidence: #508, #510, #355, #356, #358
