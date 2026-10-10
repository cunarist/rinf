---
description: Web support was deferred for years and shipped in 2.0 (2023-07) on the stance that native quality must never be compromised for web
---

Issue #34 (a highly reacted request) laid out the web limits: no native threads, atomics or time, tokio unavailable in wasm, async runtimes do not yield to JavaScript. The maintainer first waited for the WebAssembly threads proposal and said web should be a separate feature or subset. Users on mobile+web argued it matters. Web landed in one huge commit (7410bfa9, "Enable web") with a web alias module for sleep/print/spawn and wasmtimer, copying an engine into the project; see [[template-copied-library-code-forced-reruns]].

Running web needs cross-origin isolation headers, so `flutter run` for web was wrapped with HTTP headers in 2.2.0 (#77, 0b9df3dd); the CLI wasm build installs wasm-pack and nightly itself, so CI must not install them manually. Details: [[wasm-shared-memory-headers]], [[web-wide-integers]].

Evidence: #34, #77, #87, commits 7410bfa9, 43e81413, 0b9df3dd, 04699a6f, a3aed898
