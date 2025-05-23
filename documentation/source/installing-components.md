# Installing Components

[flutter-install-steps]: https://docs.flutter.dev/get-started/install

To get started, you need to have [Flutter SDK][flutter-install-steps] and [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system.

Once you're done with the installations, verify your system's readiness with the following commands. Make sure you have installed all the subcomponents that Flutter suggests. If there are no issues in the output, you are good to go onto the next step!

```{code-block} shell
:caption: CLI
rustc --version
flutter doctor
```

```{warning}
If you're working on Linux, it is recommended to install Flutter manually. Do not install Flutter from `snap`.

Flutter from `snap` comes with its own binary linker called `ld`, which is fundamentally incompatible with Rust. Instead, follow the [manual installation steps per the Flutter docs][flutter-install-steps].

Typically, the Flutter SDK package found in Debian-based Linux distributions' app store (under the name "Flutter") is a `snap` package.
```
