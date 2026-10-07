---
description: Malformed generated Dart (extra braces, broken classes) usually comes from Rust syntax edge cases like trailing commas; reduce to a small reproducer first
---

Invalid Dart output has often come from edge cases in Rust syntax tracing or
formatting, such as trailing-comma handling. When Dart output has extra braces
or malformed classes, reduce the Rust signal shape to a small reproducer and
verify the generated Dart before broad refactors.

Enum output deserves extra attention: fallback behavior and Dart exhaustiveness
have historically been a source of generated-code bugs.
