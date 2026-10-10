---
description: External PRs cluster around platform support, Cargokit, and toolchains; treat them as high-value but high-regression-risk
---

External PRs usually cluster around platform support, Cargokit/build
integration, and toolchain compatibility. Treat these changes as high-value but
high-regression-risk, because they often touch platforms the primary maintainer
may not be actively using.

Areas that came from contributors and must be kept covered:

- Early Cargokit, Xcode, and CI work established much of the platform-build
  foundation.
- Windows ARM64 support came through external platform expertise; keep it
  covered when touching Cargokit or target lists.
- eLinux and ohos support came through targeted platform work; preserve them as
  platform surfaces even when local coverage is unavailable.
- File-based endpoint and Dart analyzer fixes are recurring examples of
  contributor-driven polish.

See [[cargokit-preserve-upstream-history]].

Policy details: Cargokit changes used to go upstream first and are now patched in place ([[cargokit-preserve-upstream-history]]); new platforms are labelled experimental and the contributor is asked to help stabilise them ([[ohos-support-is-experimental]]). Contributors carry their own platform validation.

Evidence: #435, #665
