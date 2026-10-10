---
description: Since tokio_with_wasm 0.5, users import `tokio_with_wasm::alias as tokio` and must enable features like `time` on their own tokio dependency
---

A user's tests lost the `time` feature after the upgrade. The 0.5 line changed the import to the alias module; Rinf bumped to 0.5.x in #365.

Evidence: #371, #365, #367
