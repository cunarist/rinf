At the start of every new session, run `damem recall` in the terminal and treat
its output as the context for this repository. If the `damem` command is not
found, read [INSTALL.md](https://github.com/cunarist/damem/blob/main/INSTALL.md)
and follow it before reasoning or responding.

# AGENTS.md

## Project

- **Rinf** — Rust↔Flutter bridge via FFI/wasm
- Rust crate (`rust_crate`), proc macros (`rust_crate_proc`), CLI (`rust_crate_cli`), Flutter plugin (`flutter_package`)

## Rules

- **Actively maintain** memory and skills:
  - Create, update, or delete them whenever you learn something new
  - Even without explicit user command — if it matters, persist it
- Code changes must pass `cargo clippy --all-targets` + `dart analyze`
- Use conventional commits (`feat:`, `fix:`, `chore:`, `docs:`)

## Workflow

- **Signal changes:** Add `#[derive(SignalPiece)]` + `#[signal]`, run `rinf gen`, test target platform
- **Platform build issue:** Use the `platform-builds` skill
- **CI failure:** Clippy → Ruff → Dart analyzer, in that order
- **Dependabot PR:** Minor = auto-approve if CI green; Major = manual review

## Gotchas

- Web: test 64-bit/wide integer signals explicitly; use String for exact IDs/timestamps if JS precision matters
- `rinf gen --watch` is broken (#682) — don't use it
- Serde `with` attribute is banned on signals
- Recursive types: use `Option<Box<T>>` pattern
