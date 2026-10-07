---
description: `#[serde(skip)]` is the only allowed serde field attribute on signals; `with`, custom (de)serializers, `flatten`, and one-sided skips are banned at compile time
---

`#[serde(skip)]` is supported for fields that should be excluded from signal
checking and generated Dart output.

Custom serde behavior such as `with`, custom serialize/deserialize functions,
`flatten`, and one-sided skip attributes is banned for signal fields. The
generator cannot faithfully reproduce those transformations in Dart, so the
project catches them at compile time instead of allowing silent wire-format
drift.
