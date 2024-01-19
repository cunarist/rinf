# Installing Components

To get started, you need to have [Flutter SDK](https://docs.flutter.dev/get-started/install) and [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system.

Once you're done with the installations, verify your system's readiness with the following commands. Make sure you have installed all the subcomponents that Flutter suggests. If there are no issues in the output, you are good to go onto the next step!

```bash title="CLI"
rustc --version
flutter doctor
```

> If you're working on Linux, do not install Flutter from `snap`. Flutter from `snap` comes with its own binary linker called `ld`, which is fundamentally incompatible with Rust. Instead, follow the manual installation method as written in the Flutter docs.
