---
description: Each language frees the memory it allocated (6.9.1); Rust copies Dart-allocated buffers instead of adopting them with `from_raw_parts`
---

Before 6.9.1, Dart malloc'd message and binary buffers and Rust took ownership with `Vec::from_raw_parts`, freeing them on drop. PR #325 changed this for clearer memory safety: Dart frees its own buffers and Rust copies from a slice. CHANGELOG 6.9.1: "the memory allocation and drop is done in each language". Rust-to-Dart binary is still an ownership handoff; the Dart-to-Rust copy was later reduced by leaf calls, see [[dart-3-5-floor-comes-from-leaf-calls]].

Related report: on Rust 1.78 a user hit an unsafe-precondition abort (`NonNull::new_unchecked requires that the pointer is non-null`) on 6.7.0; the maintainer pointed to 6.8.0, and the follow-up errors were version skew ([[cli-version-skew]]). Never mix allocators across the boundary.

Evidence: #325, #371, CHANGELOG 6.9.1
