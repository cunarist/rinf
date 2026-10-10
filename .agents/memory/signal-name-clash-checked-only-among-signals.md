---
description: `rinf gen` must check name clashes only among signals; non-signal types in different modules may share a name
---

In 8.7.0 a signal and a non-signal type with the same name in different modules aborted generation, depending on module-processing order, so it worked on one OS and failed on another. Fixed by checking names only for signals (#626).

Evidence: #626
