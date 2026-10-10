---
description: Cargokit builds with stable unless `cargokit.yaml` sets a toolchain; nightly users must set both `cargo.debug.toolchain` and `cargo.release.toolchain`
---

A user with only nightly installed hit "toolchain stable-... is not installed" on Android. Cargokit then installed stable itself and std for the Android targets was missing. Setting the toolchain in `native/hub/cargokit.yaml` works, but the old FAQ showed only the release key, so debug `flutter run` still used stable. The working config sets `cargo.debug.toolchain: nightly` and `cargo.release.toolchain: nightly`. The template deliberately ships no `cargokit.yaml` because most users use stable; the nightly setup is documented instead. Cargokit's own docs (architecture.md) define these settings.

Evidence: #147, #319
