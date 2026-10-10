---
description: Rust code denies `unwrap`/`expect`/wildcard imports via Clippy (also in the user template) and uses `match` instead of `let else`; let chains are allowed
---

During the 8.0 cleanup `unwrap_used`, `expect_used` and `wildcard_imports` were set to deny in library crates (7c74e3d0, 99b9e1a6, a32d796e) and added to the generated user `Cargo.toml` (705863de), so scaffolded apps inherit them. `thiserror` was dropped for hand-written `AppError`/`SetupError` (814f8990, 5dceba37). 8.2 replaced `let else` with `match` (e8620261).

On 2026-01-27 an attempt to remove let chains for older compilers (1da0d7af) was reverted (d63a54f1) in favor of documenting a minimum Rust (10f90113, #661; the floor was 1.88 then, now 1.91, see [[rust-floor-1-91-and-runner-pinning]]). Do not rewrite let chains to chase older compilers. See [[no-panic-across-boundary]].

Evidence: commits 7c74e3d0, 705863de, 814f8990, e8620261, 1da0d7af, d63a54f1, 10f90113; PR #661
