---
description: Local workspace patching hides proc-macro dependency version mismatches that only appear once packages are published
---

Version bumps need attention to the `rust_crate_proc` dependency version
declared in the core crate. Local workspace patching can hide mismatches that
only appear when publishing or consuming released packages, so inspect that
version explicitly during every bump even when local builds pass.

See [[cli-version-skew]] and the `versioning` skill.
