---
description: Dependabot only watches GitHub Actions; pub and cargo were removed because library dependency bumps silently raised the Dart/Rust floors users must meet
---

Rinf is a library, so a merged dependency bump changes the toolchain every user needs. `flutter_lints` 6.0.0 (dependabot, 80e0e880) was followed by a Dart 3.8 SDK bump (2cb65131), so 8.5.0 to 8.8.1 required Dart 3.8 though Dart 3.5 was the intended floor. `bevy_ecs` 0.18 had to be lowered to 0.17.3 (814ec679) to keep the then Rust floor.

A user asked why 8.5 raised the Dart requirement (#637). On 2026-01-27 the maintainer reverted the SDK bump (63bfa581), documented supported versions per release in `installing-toolchains.md` (8.9.0), and removed `pub` and `cargo` from `.github/dependabot.yaml` (13d1c13f). Do not re-add them. After any dependency or lint-package bump, check `pubspec.yaml` `sdk:` and `rust-version`. Current floors: [[rust-floor-1-91-and-runner-pinning]].

Evidence: #637, #661, commits 80e0e880, 2cb65131, 63bfa581, 13d1c13f, 814ec679
