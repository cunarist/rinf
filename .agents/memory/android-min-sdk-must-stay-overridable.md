---
description: The Android plugin's sdkVersion must stay overridable by the parent app, and Gradle snippets must tolerate missing properties
---

NDK 26 rejected minSdk 16 (CXX1110). The fix PR let the app override the default, but CI caught a Groovy `Math.max(null, null, 31)` failure when properties were missing, so the patch was reworked. Keep the Android template config in line with the latest Flutter defaults (#358).

Evidence: #355, #356, #358
