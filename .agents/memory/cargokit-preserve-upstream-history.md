---
description: Cargokit syncs must keep upstream history; do not flatten or hide it unless the maintainer explicitly asks
---

Cargokit updates have a long history and should preserve upstream context.
Avoid flattening or hiding upstream history during sync work unless the
maintainer explicitly asks for that shape; lost context makes later platform
regressions harder to explain and revert.

See the `cargokit` skill and [[external-prs-are-platform-risk]].
