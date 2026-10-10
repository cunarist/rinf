---
description: A 'Could not find package rinf or file rinf' error from the CLI means the working directory is not a Flutter project with the rinf package added
---

In #193 a user ran `cargo install rinf` and then `rinf` and got `Could not find file rinf`. The maintainer's diagnosis: the error occurs when the working directory is not the Flutter project root (where `pubspec.yaml` is) or when `flutter pub add rinf` has not been run yet. The docs were reorganized so the install order is explicit, but a later commenter reported that `rinf --help` outside a project still printed `Could not find package rinf or file rinf`. Treat that message as a setup-order or cwd problem, not a broken install. A related newcomer failure on #167 (Cargokit could not open `native/hub/Cargo.toml`) was simply that the template had never been applied to the project.

Evidence: #193, #167
