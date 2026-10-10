---
description: Gradle 9 removed `project.exec`, so Cargokit's Android plugin failed with "Could not find method exec()"; fixed in Rinf 8.11.0 via injected `ExecOperations`, and a Kotlin Gradle Plugin warning remains
---

Symptom (#647, Rinf 8.7.2, Gradle 9 era): `flutter build apk` fails at `:rinf:cargokitCargoBuildHubRelease` with `Could not find method exec() for arguments [CargoKitBuildTask$_build_closure1...]` in `cargokit/gradle/plugin.gradle`. The maintainer could not reproduce because their Ubuntu Android CI still used an older Gradle, so a green CI does not prove Gradle 9 compatibility.

Fix (#694, released as 8.11.0 in #697): `CargoKitBuildTask` now injects `ExecOperations` and calls `execOperations.exec`. It was applied directly in the vendored Cargokit because upstream is archived, see [[cargokit-preserve-upstream-history]].

Not fixed: the plugin still applies the Kotlin Gradle Plugin, which will break future Flutter versions; it needs migration to Built-in Kotlin (Flutter's "migrate-to-built-in-kotlin" guide for plugin authors).

Evidence: #647, #694, #697
