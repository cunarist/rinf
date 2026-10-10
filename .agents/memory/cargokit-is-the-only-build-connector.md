---
description: Corrosion was removed in 2.9.0; Cargokit alone builds Rust on every platform, as a staticlib on Apple and a cdylib elsewhere
---

Before 2.9.0 Linux and Windows used Corrosion (CMake) and Android, macOS and iOS used Cargokit. Corrosion was only a leftover of the draft stage; Cargokit covers all desktop and mobile targets. A submodule for Corrosion was first rejected (#85) because the project used Corrosion's v0.3 branch for older CMake and a locally modified Cargokit. The pure Cargokit work (#99, #114) used a subtree as a transition, with upstream irondash/cargokit#10 opened for manifest-path problems. CHANGELOG 2.9.0: "Removed `corrosion`".

Apple targets stay on a static library with the link step in the podspec, because Xcode does not reliably relink the right dylib when a cdylib rebuilds under CocoaPods (explained by the Cargokit author in #95). Other platforms use cdylib. Because most build issues trace back to Cargokit, look there first. See [[cargokit-preserve-upstream-history]].

Evidence: #85, #95, #99, #114, CHANGELOG 2.9.0, commits 0335594c, e6422712
