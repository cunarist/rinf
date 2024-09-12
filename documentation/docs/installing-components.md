# Installing Components

To get started, you need to have [Flutter SDK](https://docs.flutter.dev/get-started/install)[^1] and [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system.

[^1]: If you're working on Linux, do not install Flutter from `snap`. Flutter from `snap` comes with its own binary linker called `ld`, which is fundamentally incompatible with Rust. Instead, follow the manual installation method as written in the Flutter docs.

Then, activate some necessary features:

```bash title="CLI"
flutter config --enable-native-assets
dart pub global activate native_doctor
```

Once the installations are complete, verify your system's readiness with the following commands:

```bash title="CLI"
rustc --version
flutter doctor
dart pub global run native_doctor
```

Ensure all suggested subcomponents are installed. If no issues appear in the output, you’re ready to move on to the next step!
