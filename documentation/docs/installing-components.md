# Installing Components

> This section assumes that [Flutter SDK](https://docs.flutter.dev/get-started/install) is installed on your system.

Installing Rust toolchain is very easy. Just head over to the [official installation page](https://www.rust-lang.org/tools/install) and follow the instructions.

You also need to have Protobuf compiler installed on your system. For those who aren't familiar, Protobuf is a popular, language-neutral, binary serialization format for structured messages, made by Google. Installing Protobuf compiler is also easy as described in the [official docs](https://grpc.io/docs/protoc-installation/).

After installation, verify your system's readiness with the following commands. Make sure you have installed all the subcomponents that Flutter suggests. If there are no issues in the output, you are good to go onto the next step!

```bash
rustc --version
protoc --version
flutter doctor
```
