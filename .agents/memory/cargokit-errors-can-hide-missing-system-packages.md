---
description: A Linux build that dies with 'Target of URI doesn't exist: package:build_tool/build_tool.dart' can really be a missing system package such as perl-Digest-SHA on Fedora
---

In #273 a Fedora 39 user saw `Exception: Build process failed` and, from `dart analyze`, the error `Target of URI doesn't exist: 'package:build_tool/build_tool.dart'` inside `cargokit_build/tool/bin/build_tool_runner.dart`. The maintainer could not reproduce on Ubuntu with Rinf 5.4.0, and neither could CI. The reporter found the real cause in another Flutter plugin's tracker (super_native_extensions#257): the system package `perl-Digest-SHA` was missing (`sudo dnf install perl-Digest-SHA`). The `build_tool` URI error is only a symptom of the failed pub resolution step.

Because the maintainer could not reproduce it, PR #275 added a GitHub Action that checks installation from a user's perspective. Reports that CI passes but a user fails should first compare distro packages, not Rinf code.

Evidence: #273, #275
