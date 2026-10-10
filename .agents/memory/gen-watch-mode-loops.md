---
description: `rinf gen --watch` regenerates forever on its own or unrelated file events (#682), so use one-shot `rinf gen`
---

`rinf gen --watch` has a history of detecting its own changes or unrelated
filesystem events and regenerating indefinitely (issue #682). Use one-shot
generation unless watch mode has been specifically fixed and revalidated.

Before 8.0 the old `rinf message -w` crashed with `StdinException: Error setting terminal echo mode` when stdin was not a TTY (Makefile, background job, CI) because it toggled `stdin.echoMode` (#441); any future watch mode must not assume an interactive terminal.

See [[generator-input-noise]].
