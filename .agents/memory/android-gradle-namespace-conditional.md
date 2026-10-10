---
description: The Android plugin sets `namespace` only when the host Gradle supports it, because AGP 8 requires one and older AGP rejects it
---

Apps on Android Gradle Plugin 8.0.0 failed with "Namespace not specified" for the rinf module (#242). The fix (PR #243, 4.20.0) added `namespace 'com.cunarist.rinf'` inside `if (project.android.hasProperty("namespace"))` in the plugin `build.gradle`, so older AGP still works. Reference: Flutter issue 125181.

Evidence: #242, PR #243, commit bfee8df7, CHANGELOG 4.20.0
