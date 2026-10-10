---
description: The sample crate disables `reqwest` on Android because openssl-sys is hard to build there, especially from Windows
---

The 4.14.0 sample used `reqwest`, which pulls in `openssl-sys` and needs a C OpenSSL. Compiling for Android on Windows failed (#206). PR #207 (4.15.1) disabled it for Android, and 4.15.2 switched the example URL to `http`. Keep sample dependencies buildable on all six platforms without extra system setup.

Evidence: #206, #207, CHANGELOG 4.14.0, 4.15.1, 4.15.2
