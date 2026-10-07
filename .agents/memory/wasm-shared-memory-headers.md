---
description: Web builds need nightly/std WASM support plus shared-memory flags, and shared memory needs COOP/COEP headers passed as Flutter web headers
---

Web builds are sensitive because Rinf relies on WASM features that require
nightly/std build support plus shared-memory-related flags. Shared memory also
requires COOP/COEP headers at runtime; pass them as Flutter web headers during
local runs. The published documentation cannot set headers, so it attaches
them from `documentation/source/_extra/coi-serviceworker.js` instead.

See [[wasm-heap-base-export]] and [[web-wide-integers]].
