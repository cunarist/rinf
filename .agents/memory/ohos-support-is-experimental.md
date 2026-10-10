---
description: ohos (HarmonyOS) support is experimental, rides on Cargokit, has no CI because no ohos Flutter template exists, and is kept by its contributor
---

ohos support came from a contributor (#665, issue #637) who agreed to maintain it. It could not be exercised in CI, needed a Cargokit sync first (#663), and the ohos Flutter fork lags (Flutter 3.27 / Dart 3.6 at the time). That user complained when 8.5 raised the Dart requirement, which led to explicit minimum Dart/Rust/Flutter versions per release in `installing-toolchains` docs (8.9.0) and to [[dependabot-limited-to-github-actions]]. Expect it to break when moving to Native Assets, see [[native-assets-migration-roadmap]].

Other nonstandard platforms follow the same pattern: community PRs, reviewed slowly, labelled experimental, contributor asked to help stabilise them (eLinux #435, flutter-pi #338). See [[external-prs-are-platform-risk]].

eLinux (#435) was only tested on x86, cross compiling untested; the maintainer asked for a Raspberry Pi with a small display as test device and took the change directly after waiting on upstream irondash/cargokit (see [[cargokit-preserve-upstream-history]]). A contributor's music app stayed on Rinf 7 because it used Protobuf for a remote-control protocol between devices, which SignalPiece does not cover.

Evidence: #637, #663, #665, #678, #435, #338
