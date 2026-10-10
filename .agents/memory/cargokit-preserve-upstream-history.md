---
description: Rinf patches its vendored Cargokit directly because upstream is archived; keep upstream history intact when syncing and keep local edits small and documented
---

Cargokit is vendored under `flutter_package/cargokit` via git subtree. Until 2026 the policy was upstream first: send changes to irondash/cargokit and pull them in, bypassing only when upstream review stalled about a month (eLinux #435, Android x86 #437, rustup assumption #620). That is no longer current. The upstream author said (irondash/cargokit#115) that no more work is planned and that Cargokit is obsolete with Dart native assets.

Rinf now patches its vendored copy directly: ohos (#665), Gradle 9, which removed `project.exec` (#694), and Windows ARM64 (#650, issue #649). Still sync with upstream if it moves, do not flatten or hide upstream history unless the maintainer asks, and keep local edits minimal and documented. The long-term replacement is Dart Native Assets, see [[native-assets-migration-roadmap]]; SwiftPM PR #675 was rejected and #681 is open. Early commits here are Cargokit's own, see [[early-git-history-is-cargokit-upstream]].

See the `cargokit` skill, [[external-prs-are-platform-risk]], [[cargokit-assumes-rustup-nix-needs-shim]].

Evidence: #435, #437, #620, #649, #650, #665, #675, #681, #694, irondash/cargokit#115
