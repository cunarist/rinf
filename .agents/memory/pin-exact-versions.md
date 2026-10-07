---
description: Pin exact versions (runner images, toolchains, actions, tools) instead of latest, main, master, or stable
---

Always prefer an exact version over a floating one such as `latest`, `main`,
`master`, or `stable`. Floating references move without any change in this
repository and break CI or builds overnight.

- CI runners use exact images (`ubuntu-24.04`, `windows-2022`, `macos-26`),
  never `*-latest`. `windows-latest` moved to `windows-2025` with only Visual
  Studio 2026, which the minimum supported Flutter 3.24 cannot find.
- Rust toolchains use an exact version (`dtolnay/rust-toolchain@1.91`), never
  `@master` or `@stable`.
- When bumping a pinned version, research what the new one ships first. See
  [[research-before-every-change]].

The exception is the scheduled User App workflow, which deliberately follows
the latest stable Flutter and Rust to catch ecosystem breakage before users hit
it.
