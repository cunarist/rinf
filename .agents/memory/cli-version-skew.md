---
description: The CLI is versioned and installed apart from the framework crates, so check CLI/package version alignment first when `rinf gen` output looks inexplicable
---

The CLI (`rust_crate_cli`) is intentionally versioned and installed
independently from the core framework crates, so the tool can evolve without
forcing every framework dependency to move in lockstep.

The cost is version skew: if the installed `rinf gen` does not match the
package version used by the Flutter app, generation can behave strangely. This
is a recurring source of confusing output. Before debugging the AST logic or
generator internals, confirm the installed CLI and the Flutter package
configuration are aligned.

The CLI crate is also separated from the main workspace and is checked and
installed with its own lock file in some CI workflows; do not assume workspace
lock behavior covers CLI installation.

See [[proc-macro-version-hidden-by-patching]].

Typical report: after bumping only `pubspec.yaml` (7.3.0, 7.3.1) Rinf stops working on every platform and `main` is never executed; it works once `native/hub/Cargo.toml` is bumped to the same version (#521, #543; one reporter git-bisected before finding it). 7.3.0 changed who calls `store_dart_post_cobject` ([[store-dart-post-cobject-called-from-rust]]). A stale CLI against a newer crate gives compile errors such as `no SharedCell in the root`, or `Option<Vec<_>>` mismatches with old generated files; the fix is `cargo update`, reinstall the CLI, `flutter pub upgrade`, rerun generation (#411).

The maintainer called a version-mismatch message "definitely a good idea" but no such check exists in `rust_crate_cli/src`; `documentation/source/upgrading.md` is the only guard. First question for any "nothing works after upgrade" report: are pubspec, Cargo.toml and the installed CLI identical? Since 8.0 the CLI is the separate `rinf_cli` crate (5d36c8d1).

Users upgrading one side also see `no SharedCell in rinf` or `mismatched types Option<Vec<_>>`; the fix is the same version in `pubspec.yaml` and `Cargo.toml`, a fresh `cargo install rinf`, and rerunning generation. Pinning an old crate by git tag can hit crates.io lacking that version (#371).

Evidence: #371, #411, #521, #543, commits a1df9707, 5d36c8d1
