---
description: 64-bit and wider integer signal fields must be tested on web, and exact IDs, counters, or timestamps should be strings there
---

Rinf supports 64-bit and wider integer types in the Rust traits and generator,
but on web the values cross JavaScript and generated Dart code, so web/WASM is
a practical precision boundary. Test schemas with such fields on web
explicitly, even when generation succeeds. Use `String` for exact IDs,
counters, and timestamps that must keep full precision, or `double` only when
precision loss is acceptable.

See [[wasm-shared-memory-headers]].
