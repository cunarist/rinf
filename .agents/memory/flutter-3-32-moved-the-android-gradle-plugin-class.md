---
description: Flutter 3.32 moved its Gradle plugin class to `com.flutter.gradle`, so old Cargokit hooks silently built nothing and apps crashed with a missing `libhub.so`; fixed in the vendored Cargokit by #606
---

On Flutter 3.32 Android apps failed at runtime with `libhub.so` not found (#605, linked to irondash/cargokit#93). The Cargokit Gradle plugin looked for a plugin named `FlutterPlugin` and called `plugin.getTargetPlatforms()`; Flutter 3.32 renamed the class to `com.flutter.gradle.FlutterPlugin` and moved the platform lookup to `com.flutter.gradle.FlutterPluginUtils.getTargetPlatforms(project)`. PR #606 (merged 2025-05-23) changed both lines in `flutter_package/cargokit/gradle/plugin.gradle`.

The maintainer's stance at the time: update Cargokit manually if upstream does not fix it. When a new Flutter release produces `libhub.so not found` on every Android build (not only a missing rustup target, see [[android-missing-rust-target-libhub-not-found]]), diff Flutter's Gradle plugin API against `cargokit/gradle/plugin.gradle` first.

Evidence: #605, #606
