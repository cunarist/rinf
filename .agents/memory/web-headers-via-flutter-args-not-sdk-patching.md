---
description: Since Rinf 7 the CLI no longer patches the Flutter SDK for web COOP/COEP headers; users pass headers as `flutter run` arguments and `rinf server` prints the command
---

In Rinf 5/6 the web setup edited the Flutter SDK (`devfs_web.dart`) to inject the headers shared-memory WASM needs. It broke on read-only SDKs such as NixOS (#253) and some Linux distributions (6.6.1 added a workaround; `flutter upgrade --force` undoes patches). Writing a custom dev server was rejected as overengineering because Flutter debug tooling would be lost; the maintainer waited for Flutter's custom web headers (flutter/flutter#136297, released in Flutter 3.19).

PR #415 (84a60afe, Sep 2024) removed all patching, helped by Dart 3.5/Flutter 3.24; 7.0.0 added `rinf server`, which prints the full `flutter run` command with headers. If web fails at startup, check headers before Rust. Never reintroduce SDK patching. See [[wasm-shared-memory-headers]], [[dart-3-5-floor-comes-from-leaf-calls]].

Evidence: #253, #214, #378, #415, #408, commit 84a60afe, CHANGELOG 6.6.1, 7.0.0
