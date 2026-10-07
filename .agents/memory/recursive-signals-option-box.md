---
description: Recursive signal types must use `Option<Box<T>>`, which gives the generator a finite nullable edge
---

Recursive signals should use the `Option<Box<T>>` pattern. The nullable edge is
the important part: it gives the generator a finite type shape while still
expressing recursive data. Direct recursion will not.

See [[no-generic-signal-types]].
