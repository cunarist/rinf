---
description: Empty byte vectors are sent as null and rebuilt on the Dart side because zero-copy transfer of empty buffers is unsafe
---

Empty byte vectors have historically been unsafe for zero-copy transfer on the
Dart boundary. The bridge sends them as null and reconstructs empty buffers on
the Dart side. Do not "simplify" this path without testing empty payloads.

Root cause: the `zero-copy` feature of `allo-isolate` crashed on empty `Vec<u8>` (Android aborted after a few seconds; web was unaffected). 4.2.1 sent `None` instead of `Some(Vec::new())`, including in the default response; the maintainer sent a fix upstream (allo-isolate PR 48) and 0.1.20 fixed it (4.4.0, 7d4b7750). Users on old templates kept hitting it ([[template-copied-library-code-forced-reruns]]).

Evidence: #165, commit 7d4b7750, CHANGELOG 4.2.1, 4.4.0
