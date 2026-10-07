---
description: `rinf gen --watch` regenerates forever on its own or unrelated file events (#682), so use one-shot `rinf gen`
---

`rinf gen --watch` has a history of detecting its own changes or unrelated
filesystem events and regenerating indefinitely (issue #682). Use one-shot
generation unless watch mode has been specifically fixed and revalidated.

See [[generator-input-noise]].
