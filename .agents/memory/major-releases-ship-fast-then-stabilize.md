---
description: Majors 6, 7 and 8 went from alpha to final in days and were followed by weeks of API-shape fixes; expect breaking follow-ups and document every break
---

Tags: 6.0.0-alpha 2024-01-17 to final 2024-01-22; 7.0.0 alpha 2024-09-18 to final 2024-09-21; 8.0.0 alpha 2025-03-30 to final 2025-04-02. After 6.0 came 6.3.0 deleting the `bridge` module and 6.13.0 with breaking changes in a minor. After 7.0 came a web fix in 7.1.0 and a symbol change in 7.3.0. After 8.0 came 8.1.0 (`DartSignal`/`DartSignalBinary` non-generic, #571), 8.2.0 (trait bounds) and 8.3.0 (async signatures). Minor releases have also raised toolchain floors ([[dependabot-limited-to-github-actions]]).

Migration guides live in `documentation/source/upgrading.md`. Document every breaking change in the CHANGELOG and the upgrade page, even in minor releases. The 7-to-8 upgrade requires `cargo uninstall rinf; cargo install rinf_cli` since the CLI became a separate `rinf_cli` crate (5d36c8d1); see [[cli-version-skew]].

Evidence: git tags v6.0.0-alpha to v8.3.0, PR #571, commits 5cdc51b6, 8746ca4d, 5d36c8d1; CHANGELOG 6.3.0, 6.13.0, 8.1.0, 8.2.0
