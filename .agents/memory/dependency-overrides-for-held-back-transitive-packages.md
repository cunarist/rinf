---
description: Reports that `flutter pub add rinf` holds back transitive packages (async, vm_service, leak_tracker) were answered by Rinf 8 shrinking its pub dependencies, not by loosening constraints
---

On Rinf 7.3.0, `flutter pub add rinf` printed "5 packages have newer versions incompatible with dependency constraints". The reporter's workaround was `dependency_overrides` in `pubspec.yaml` for async, material_color_utilities, fake_async, leak_tracker and vm_service. The maintainer called it important but did not patch 7.x, and later answered that version 8 "vastly reduced the number of pub.dev dependencies" so conflicts should be much rarer (chalkdart removal was announced on #538).

Rule: do not try to fix such reports by widening ranges. Keep the Dart dependency list minimal (see [[dependabot-limited-to-github-actions]] for why bumps are not automated).

Evidence: #528, #538
