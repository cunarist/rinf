---
description: Cargokit always builds armv7 for Android (and extra targets in debug) and ignores Gradle ABI exclusion; 32-bit Android is unsupported
---

A 32-bit Android phone aborted (SIGABRT) on signal send; the answer was that 32-bit Android is unsupported. Cargokit always builds armv7 and ignores Gradle ABI exclusion, and its Gradle plugin hardcodes extra targets in debug (i686 even on an x86_64 emulator, in `cargokit/gradle/plugin.gradle`). Crates that fail on those targets (extism, lancedb) must be guarded with `#[cfg(not(target_arch = ...))]` placeholders.

i686 support (#437, upstream irondash/cargokit#111) was handled when Cargokit still went upstream first, see [[cargokit-preserve-upstream-history]].

Evidence: #368, #382, #341, #437
