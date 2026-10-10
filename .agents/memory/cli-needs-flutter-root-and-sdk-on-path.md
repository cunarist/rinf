---
description: Rinf CLI commands fail to resolve the Flutter SDK when `FLUTTER_ROOT` is unset (AUR installs); `~/.pub-cache/bin` and `~/.cargo/bin` must be on PATH
---

The error came from Dart pub being unable to find the Flutter SDK (a flutter_test sdk dependency could not resolve), not from Rinf. Arch/AUR installs do not define `FLUTTER_ROOT`; setting it fixed it. The maintainer declined switching the CLI to `flutter pub run` because it prints a deprecation notice. See [[flutter-linux-install-must-be-manual]].

Evidence: #336
