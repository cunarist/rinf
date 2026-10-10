---
description: Library logic copied into user projects by `template` forced users to rerun it on upgrades; the bridge engine was moved into the `rinf` crate (4.16.0) to stop that
---

In 2023 many releases changed files copied into the user's project, and the CHANGELOG repeatedly says to run the template command again (3.0.0, 4.0.0, 4.6.0, 4.7.0, 4.11.0). Fixes such as the empty `Vec<u8>` crash (4.2.1) needed a re-apply, so users on old templates kept reporting already-fixed bugs (#165).

Mitigations: a `--bridge` option to refresh only the bridge module (4.8.0), then moving the bridge engine into the published `rinf` crate (4.16.0, PR #212, commit c1abf165). Web support (2.0.0) had first been added by copying an engine into the project (7410bfa9), later merged into the user's `hub` crate (b646e401).

Lesson: keep library logic in dependencies, never in generated or copied user files. A user-side template change is a breaking change even in a minor version. See [[major-releases-ship-fast-then-stabilize]].

Evidence: #165, PR #212, commits 7410bfa9, b646e401, c1abf165, d60d8f57, CHANGELOG 4.8.0, 4.16.0
