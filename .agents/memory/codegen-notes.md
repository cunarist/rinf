# Code Generation Notes

## Watch Mode

Avoid `rinf gen --watch`. It has a history of detecting its own changes or unrelated filesystem events and regenerating indefinitely. Use one-shot generation unless watch mode has been specifically fixed and revalidated.

## Input Noise

The generator has historically read files that are not meaningful Rust signal sources when they appear under watched/input areas. If generation starts reacting to files like package manager metadata, add those files to the ignore configuration rather than changing signal definitions.

## Output Shape Failures

Invalid Dart output has often come from edge cases in Rust syntax tracing or formatting, such as trailing-comma handling. When Dart output has extra braces or malformed classes, reduce the Rust signal shape to a small reproducer and verify generated Dart before broad refactors.

## Version Skew

Generator/package version mismatch is a recurring source of confusing output. Before debugging code generation internals, confirm the installed CLI and the Flutter package configuration are aligned.

Version bumps also need attention to proc-macro dependency versions. Local workspace patching can hide mismatches that only appear when publishing or consuming released packages.

## Signal Attribute Wart

There is historical confusion around Rust-signal attribute extraction for binary and non-binary Rust-to-Dart signals. Before refactoring that logic, generate both signal kinds and inspect the Dart assignment/stream behavior.
