---
description: Generated Dart serialization bugs (lints, enum matching, web i64, `Vec<char>`) are fixed upstream in serde-reflection, then bumped; Rinf only fixes its own naming code
---

The maintainer's stance: generated serialization code belongs to zefchain/serde-reflection. Examples: lint warnings in `bincode_serializer.dart` (#632), abstract vs sealed enums for exhaustive switch (#638), web `getInt64`/`setInt64` throwing silently so signals with i64 never arrive (#672, serde-reflection#92; the exception occurs inside `assignRustSignal` and nothing is logged, see [[web-wide-integers]]). `Vec<char>` fails with `RangeError` in `deserializeChar` (reads an Int64); use `String` instead (#619, confirmed bug).

Local fixes are for Rinf's own code: snake-case conversion of names with digits like `Uint64Msg` (#600, fixed in #604 by using the same case converter) and an extra brace (#601, #603). See [[dart-casing-follows-serde-generate]], [[malformed-dart-output]].

Evidence: #600, #601, #603, #604, #619, #632, #638, #672
