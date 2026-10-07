---
description: Empty byte vectors are sent as null and rebuilt on the Dart side because zero-copy transfer of empty buffers is unsafe
---

Empty byte vectors have historically been unsafe for zero-copy transfer on the
Dart boundary. The bridge sends them as null and reconstructs empty buffers on
the Dart side. Do not "simplify" this path without testing empty payloads.
