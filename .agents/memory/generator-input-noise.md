---
description: When `rinf gen` reacts to non-Rust files like package manager metadata, add them to the ignore configuration instead of changing signals
---

The generator has historically read files that are not meaningful Rust signal
sources when they appear under watched or input areas. If generation starts
reacting to files like package manager metadata, add those files to the ignore
configuration rather than changing signal definitions.

See [[gen-watch-mode-loops]].
