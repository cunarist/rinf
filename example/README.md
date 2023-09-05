# Example App

Demonstrates how to use the Rust-In-Flutter plugin.

# Using Rust Inside Flutter

This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rust-In-Flutter](https://pub.dev/packages/rust_in_flutter) framework.
For detailed instructions on writing Rust and Flutter together,
please refer to its [documentation](https://docs.cunarist.com/rust-in-flutter).

Messages sent between Dart and Rust are implemented using Protobuf.
If you have newly cloned the project repository
or made changes to the `.proto` files in the `./messages` directory,
run the following command:

```bash
dart run rust_in_flutter message
```
