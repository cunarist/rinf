---
description: Cargokit hard-requires rustup to launch cargo, so Nix-managed toolchains fail with misleading errors; a fake `rustup` shim is the workaround
---

Cargokit runs cargo through `rustup`. Nix users hit "rustup not found in PATH" (#620) or a misleading "unable to parse manifest" caused by a different cargo version than the one `cargo build -p hub` uses (#631). The reporter's workaround was a stub `rustup`. Cargokit's error reporting hides the real cause, so ask which `cargo` and `rustc` Cargokit actually runs.

This is fixable in the vendored copy now, see [[cargokit-preserve-upstream-history]].

Evidence: #620, #631
