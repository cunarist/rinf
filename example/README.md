# Example App

Demonstrates how to use the Rust-In-Flutter plugin.

## Using Rust Inside Flutter

This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rust-In-Flutter](https://pub.dev/packages/rust_in_flutter) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install),
[Rust toolchain](https://www.rust-lang.org/tools/install),
and [Protobuf compiler](https://grpc.io/docs/protoc-installation)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```bash
rustc --version
protoc --version
flutter doctor
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rust-In-Flutter's [documentation](https://docs.cunarist.com/rust-in-flutter).

Messages sent between Dart and Rust are implemented using Protobuf.
If you have newly cloned the project repository
or made changes to the `.proto` files in the `./messages` directory,
run the following command:

```bash
dart run rust_in_flutter message
```
