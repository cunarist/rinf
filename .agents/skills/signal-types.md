# Signal Types

Use this skill when adding or changing a type that crosses the Dart/Rust boundary.

## Core Rules

- Shared nested data derives `SignalPiece`.
- Dart-to-Rust endpoints derive the Dart signal trait variant.
- Rust-to-Dart endpoints derive the Rust signal trait variant.
- Pair signal derives with the serde direction the endpoint needs; shared DTOs usually derive both serialize and deserialize.
- Keep fields public and concrete. Generic foreign signal types are rejected by design.
- Keep framework-reserved names out of user signal types; names beginning with `rinf` are reserved.

## Supported Shapes

Prefer primitives, strings, options, boxes, arrays, vectors, sets, maps, unit, small tuples, and local structs/enums that also derive `SignalPiece`.

Use `Option<Box<T>>` for recursive structures. The nullable edge is the important part; direct recursion will not give the generator a finite type shape.

For web-safe cross-platform signals, explicitly test 64-bit and wider integer fields. Use `String` for exact IDs/timestamps or `double` only when precision loss is acceptable.

## Serde Attributes

`#[serde(skip)]` is allowed and means "leave this out of signal validation and generated Dart."

Do not use custom serde transforms, `flatten`, or one-sided skip attributes on signal fields. The Dart generator cannot reproduce those semantics reliably, so the project rejects them instead of producing a misleading API.

## Receiver Pattern

Only the newest Dart-signal receiver is active. If `recv()` returns `None`, stop that task or reacquire the receiver; it was superseded by a newer clone.

Rust-to-Dart streams retain the latest received value. When adding endpoint types, verify generated Dart exposes both stream listening and latest-value access for Rust-to-Dart signals.

## Checklist

- Run the generator after changing signal shapes.
- Analyze generated Dart.
- Run Rust clippy for behavior changes.
- Test the target platform, especially web when signal fields include numeric IDs or timestamps.
