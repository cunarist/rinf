---
description: No `unwrap`/`expect` in Rinf; bridge errors are logged and swallowed, and bad signals are dropped rather than killing the runtime thread
---

Rinf adopted a no-`unwrap`/`expect` posture because panic behavior is
especially costly around FFI and WASM boundaries. Errors should be converted
into project error types, logged, and swallowed when crossing the bridge would
otherwise destabilize the host app.

For signal transport, a bad message is dropped rather than killing the runtime
thread. This is a deliberate reliability tradeoff: bridge errors are reported,
but application lifecycles stay intact.
