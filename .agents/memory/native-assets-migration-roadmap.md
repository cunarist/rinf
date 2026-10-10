---
description: The planned build system is Dart Native Assets (`hook/build.dart` plus native_toolchain_rust) replacing Cargokit, CocoaPods and SwiftPM glue; new glue PRs are declined
---

The long-term plan is Dart Native Assets (`package:hooks` + `native_toolchain_rust`), tracked in #641 (open). The maintainer called Cargokit "a hack before Flutter native assets". Flutter is moving from CocoaPods to SwiftPM (#673, flutter/flutter#168015).

A SwiftPM PR (#675) was rejected as obsolete on arrival and fragile: a silent `git restore` PostAction, `.unsafeFlags` blocking downstream SwiftPM packages, and a hardcoded `ios-arm64` slice breaking simulators. An Apple-only Native Assets PR (#681, closes #673) is open and not merged.

Known costs from the prototype: stable Native Assets needs Flutter 3.38.1 / Dart 3.10, raises iOS minimum 12 to 13 and macOS 10.14 to 10.15, and needs the hub path in pubspec `hooks.user_defines.rinf.hub_path`. The prototype used string-based dynamic symbol lookup. Until then Cargokit stays and Cargokit-dependent platforms (ohos) are experimental, see [[ohos-support-is-experimental]] and [[cargokit-preserve-upstream-history]]. Rinf is for apps only; package authors are pointed to native_toolchain_rust, see [[rinf-is-for-apps-not-packages]].

Evidence: #641, #673, #675, #681, flutter/flutter#168015
