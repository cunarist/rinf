---
description: Windows builds have broken on cross-drive `cd`, missing shell aliases, lost executable bits, and argument escaping; keep scripts path-safe
---

Windows path handling has failed on cross-drive `cd` operations, missing shell
aliases, script executable bits lost after Windows-to-macOS moves, and argument
escaping. Keep Cargokit and script changes path-safe across shells and drives:
use explicit drive-aware shell behavior and quoted structured paths.

See [[cli-structured-paths]].
