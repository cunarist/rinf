---
description: On old Android versions the native library fails to load until android:extractNativeLibs="true" is set in AndroidManifest.xml
---

A contributor reported (#265) that on old Android the native load does not work and an error appears; setting `android:extractNativeLibs="true"` on the `<application>` element in `AndroidManifest.xml` fixes it. The maintainer put it in the FAQ (PR #266). The #280 reporter later tried that FAQ entry and it had no effect, because their failure was a missing C++ runtime, not extraction, so use it only for plain `libhub.so` load failures on old devices and see [[android-libcxx-is-users-build-rs]] otherwise.

Evidence: #265, #266, #280
