---
description: A reqwest call from Rust compiled to web that panics with "TypeError: Failed to fetch" is a browser CORS problem on the target server, not a Rinf or tokio_with_wasm bug
---

On web, `reqwest` uses the browser `fetch` API, so normal browser rules apply. A reporter's request from the Rust side died with `reqwest::Error { kind: Request ... JsValue(TypeError: Failed to fetch) }`, and the reporter later found it was CORS: the server must send correct CORS headers and answer the `OPTIONS` preflight request correctly. Nothing needed changing in Rinf.

Triage rule: when an HTTP call works natively but fails with "Failed to fetch" on web, check the server's CORS headers and preflight handling first. Note this is separate from the COOP/COEP headers Rinf itself needs for shared memory (see [[wasm-shared-memory-headers]]).

Evidence: #484
