---
description: Web builds need nightly/std WASM support plus shared-memory flags, and shared memory needs COOP/COEP headers passed as Flutter web headers
---

Web builds are sensitive because Rinf relies on WASM features that require
nightly/std build support plus shared-memory-related flags. Shared memory also
requires COOP/COEP headers at runtime; pass them as Flutter web headers during
local runs. The published documentation cannot set headers, so it attaches
them from `documentation/source/_extra/coi-serviceworker.js` instead.

See [[wasm-heap-base-export]] and [[web-wide-integers]].

COEP must be `require-corp`, not `credentialless`: `credentialless` works only in Chrome and Edge, and the demo site broke on Firefox and Safari (#164); 4.3.0 switched (9c00ce5f). Static-server deployments hit CORS and `hub.js` load failures when headers were missing (#214). Headers are passed as `flutter run` arguments, see [[web-headers-via-flutter-args-not-sdk-patching]]; docs-site handling is in [[docs-site-embeds-example-web-app]].

Evidence: #164, #214, commit 9c00ce5f, CHANGELOG 4.3.0
