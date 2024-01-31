# Installing Components

To get started, you need to have [Flutter SDK](https://docs.flutter.dev/get-started/install)[^1] and [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system.[^2]

[^1]: If you're working on Linux, do not install Flutter from `snap`. Flutter from `snap` comes with its own binary linker called `ld`, which is fundamentally incompatible with Rust. Instead, follow the manual installation method as written in the Flutter docs.
[^2]: If you encounter issues with the automated `protoc` installation, likely due to GitHub API access restrictions, you can [manually install it](https://grpc.io/docs/protoc-installation/) on your machine and add it to PATH. In this case, verify the installation by running the command `protoc --version` to ensure that the Protobuf compiler is ready on your machine.

Once you're done with the installations, verify your system's readiness with the following commands. Make sure you have installed all the subcomponents that Flutter suggests. If there are no issues in the output, you are good to go onto the next step!

```bash title="CLI"
rustc --version
flutter doctor
```
