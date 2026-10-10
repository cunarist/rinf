---
description: The Android NDK version comes from the Flutter SDK, not from Rinf; pinning an NDK in the plugin caused build errors
---

CHANGELOG 1.5.0: the NDK version the Flutter SDK expects is used, not one specified by this package (f15fd247, which also updated Cargokit). Earlier guides (1.3.2) told users to specify an NDK version and needed "build tool version issues" guides. Do not pin an NDK in the plugin.

Android path detection must use Gradle variables, not environment variables: 1.1.1 failed on Windows because it relied on `PWD`, which Flutter sets on macOS only (fixed in 1.3.1).

Evidence: #61, #42, #54, commits f15fd247, f0a69969; CHANGELOG 1.3.1, 1.5.0
