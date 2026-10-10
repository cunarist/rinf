---
description: The docs landing page embeds the example app built for web in CI, which is why the docs need a COOP/COEP service worker; URLs have moved several times
---

PR #550 (Mar 2025) put the example web app on the docs landing page. In 2026 (958431ef) a `documentation/Containerfile` plus the `documentation.yaml` workflow build it in a container with `BASE_HREF` for the project subpath and publish to GitHub Pages. Pages cannot set headers, so `_extra/coi-serviceworker.js` supplies cross-origin isolation, see [[wasm-shared-memory-headers]].

`sphinx-autobuild` alone leaves the demo iframe empty; use the podman image. The docs URL moved from `rinf.cunarist.com` to `rinf.cunarist.org` to `cunarist.github.io/rinf`, so old links in issues, crates.io metadata and podspecs may be stale. Docs moved from MkDocs to Sphinx+Furo in Feb 2025 (#516). Keep docs concise: NDK or Android glue guidance goes in short FAQ entries, no examples for unstable APIs (bevy), see [[update-docs-with-every-change]].

Evidence: PR #550, #516, #494, #349, commits 0b8256bc, c93d2b4c, dd2ef96b, 958431ef
