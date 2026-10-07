---
description: Exported symbols carry a `rinf` prefix to avoid clashes between Flutter packages, so user signal names starting with `rinf` are reserved
---

Rinf added a `rinf` prefix to exported symbols and moved
`store_dart_post_cobject` setup into Rust to avoid symbol conflicts between
multiple Flutter packages.

User signal names beginning with `rinf` remain reserved. Generator validation
should apply that reserved-name rule only to actual signal types, not to every
Rust struct discovered while scanning source; a reserved-name error on an
ordinary helper struct is a generator bug.
