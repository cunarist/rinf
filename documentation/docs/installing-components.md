# Installing Components

To get started, you need to have [Flutter SDK](https://docs.flutter.dev/get-started/install), [Rust toolchain](https://www.rust-lang.org/tools/install), and [Protobuf compiler](https://grpc.io/docs/protoc-installation/) installed on your system.

> If you're working on Linux, do not install Flutter from `snap`. Flutter from `snap` comes with its own binary linker called `ld`, which is fundamentally incompatible with Rust. Instead, follow the manual installation method as written in the Flutter docs.

After the installation, verify your system's readiness with the following commands. Make sure you have installed all the subcomponents that Flutter suggests. If there are no issues in the output, you are good to go onto the next step!

```bash
rustc --version
protoc --version
flutter doctor
```

> For those who aren't familiar, Protobuf is a popular, language-neutral, binary serialization format for structured messages, made by Google.
