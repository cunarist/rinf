---
description: Generic foreign signal types are rejected on purpose because Dart type mapping becomes ambiguous; use concrete DTOs even at the cost of duplication
---

Signal types should stay inside the set that the derive macros and Dart
generator can model clearly. Containers such as `Box<T>`, `Option<T>`, arrays,
vectors, sets, maps, and small tuples are supported patterns, but generic
foreign signal types are rejected.

The generic-type ban is intentional. Dart-side type mapping becomes ambiguous,
and failures tend to surface later than Rust compile time. Prefer concrete
signal DTOs even if that creates a small amount of duplication.

See [[recursive-signals-option-box]].
