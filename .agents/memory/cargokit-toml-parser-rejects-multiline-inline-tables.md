---
description: Windows builds fail (cargokit.rule exits -1) when the app's `native/hub/Cargo.toml` has multi-line inline tables, because Cargokit's Dart `toml` package follows TOML 0.5.0; write such dependencies on one line
---

Reported in #685 (Rinf 8.10.0, Windows): a dependency like `image = { version = "...", features = [` ... `] }` spread over several lines makes the custom build step fail with exit code -1 and "Build process failed", with no useful message. The Dart `toml` 0.14.0 parser used by the vendored Cargokit implements TOML v0.5.0, which has no multi-line inline tables. Workaround: put the whole inline table on one line (or use a normal `[dependencies.image]` table). The issue is open and unanswered; the proposed fix is to update the TOML package in Cargokit's build tool.

When a build fails with "terminated with code -1" yet `cargo build -p hub` works, check the hub `Cargo.toml` for multi-line inline tables. Similar to the misleading manifest error in [[cargokit-assumes-rustup-nix-needs-shim]].

Evidence: #685
