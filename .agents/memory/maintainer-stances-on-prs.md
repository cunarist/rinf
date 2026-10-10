---
description: Maintainer preferences for PRs: single matrix CI, green main via PRs, structured config over flags, explicit codegen, and no new glue that Native Assets will replace
---

CI: one `build_test.yaml` with matrices rather than duplicated files; extra multi-arch CI (aarch64, armv7, riscv via third-party QEMU actions) was declined because guaranteed-every-arch CI rots (#112). From #89 on, main gets only green commits and everyone, including the maintainer, uses PRs. Web CI uses the project's own CLI, not manual wasm-pack installs. A Flutter formatter change can break contributors on older SDKs.

Dependencies: Cargo.lock is committed only for the CLI binary (#645), not library crates; PRs #614 and #640 asked for it for nixpkgs. Splitting the CLI out of the workspace was rejected (recompiles rinf, two target dirs). Risky crates get replaced (serde_yml flagged, move to serde_yaml_ng, #635). Interfaces: generated types use an interface class rather than a mixin so they work as type parameters (#602). `send_signal_to_dart` takes `&self` (restored to match v7, #545); for `recv` the maintainer rejected exposing internal future structs and preferred returning `impl Future` (#458). The Rust workspace stays under `native/hub` and cannot be flattened because rust-analyzer fails to find deeper Cargo.toml (#20). The package is a pure FFI plugin (`ffiPlugin: true`) with no dummy native classes, after a Flutter engineer pointed out dead native code (#51, #139).

Evidence: #20, #51, #89, #112, #139, #458, #545, #602, #614, #635, #640, #645
