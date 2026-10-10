---
description: Flutter from snap or any distro package manager (AUR, app store) breaks Rinf builds and CLI; Linux docs require the official manual SDK install
---

A fresh Ubuntu 22.04 with snap Flutter failed linking build scripts (`GLIBC_2.33 not found`, cc/ld errors) while CI passed: the snap bundles its own linker and libc, incompatible with the Rust toolchain (#201). `rinf template` failed with "Flutter SDK is not available" on Arch with the AUR package (#488, #498), and the Ubuntu app-store Flutter is a snap that is not obvious in the UI (#499). Fix is always a manual install from flutter.dev; the docs warning was widened from "snap" to all non-manual methods and made an admonition. Ask first how Flutter was installed when a Linux user reports an opaque linker error. See [[cli-needs-flutter-root-and-sdk-on-path]].

Evidence: #201, #488, #498, #499
