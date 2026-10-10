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

Rationale: `rinf gen` is static analysis, so it cannot see logic behind `serde(with)` and similar (#580). A contributor's PR supported only `skip` on purpose: handling skip_serializing, skip_deserializing and skip_serializing_if "quickly leads to implementation complexity and user confusion" (#617). The rest became compile errors in #618 (merged 2025-06-04); before that they compiled but failed at runtime.

Evidence: #580, #617, #618
