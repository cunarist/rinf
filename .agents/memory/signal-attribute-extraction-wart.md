---
description: Attribute extraction for binary vs non-binary Rust-to-Dart signals is historically confusing; generate both kinds before refactoring it
---

There is historical confusion around Rust-signal attribute extraction for
binary and non-binary Rust-to-Dart signals. Before refactoring that logic,
generate both signal kinds and inspect the Dart assignment and stream behavior.
