---
description: Removing the empty `binary` field from `DartSignal`/`RustSignal` packs (new Binary pack types) is blocked on `rinf gen` reading the Rinf version from package manifests, for backward compatibility
---

Two open maintainer-authored issues are linked. #596 proposes `DartSignalBinaryPack` / `RustSignalBinaryPack` so plain signals no longer send an empty `binary` field; it says the current design is an artifact of older Rinf versions and that #565 must be resolved first for backward compatibility. #565 asks that `rinf gen` respect the Rinf version written in package manifests ("to make Rinf more future-proof"), which would let the generator emit code matching the user's Rust and Dart crate versions. Neither was implemented as of the last read, so wire-format cleanups must stay compatible with mismatched CLI and package versions, see [[cli-version-skew]].

Evidence: #565, #596
