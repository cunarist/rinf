---
description: Commits from 2022-04 to 2023-06 are upstream Cargokit history; Rinf's own history starts 2023-07-09 as `rust_in_flutter` 1.0.0
---

The root commit (bb060194, 2022-04-11) belongs to the Cargokit author, and commits up to 2023-07-01 (renames, NDK and Gradle fixes) are Cargokit's. They were pulled in with a history merge (e599e189). Rinf's own work starts with the 2023-07-09 burst ending in `Version 1.0.0` (9db665c8).

When blaming or bisecting old build-script behavior (NDK, Gradle, CMake, pod scripts), check Cargokit upstream first. This is also why [[cargokit-preserve-upstream-history]] matters. Names of that era: [[rinf-naming-timeline]].

Evidence: commits bb060194, e599e189, 9db665c8, CHANGELOG 1.0.0
