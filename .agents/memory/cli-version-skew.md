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
