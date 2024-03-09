# Applying Rust Template

!!! note

    This section assumes that you've already created a Flutter project with `flutter create [my_app_name]`. You can get further guidance from [this awesome Flutter tutorial](https://docs.flutter.dev/get-started/codelab).

First of all, add this framework to your Flutter project.

```bash title="CLI"
flutter pub add rinf
```

Now install the command executable to easily run Rinf commands in the CLI.[^1]

[^1]: If you're curious about all the available commands, use `rinf --help`.

```bash title="CLI"
cargo install rinf
```

Then, simply run this in the command-line[^2] from your Flutter project's directory.

[^2]: If you encounter issues with the automated `protoc` installation, likely due to GitHub API access restrictions, you can [manually install it](https://grpc.io/docs/protoc-installation/) on your machine and add it to PATH. You can verify the installation by running the command `protoc --version` to ensure that the Protobuf compiler is ready on your machine. Rinf will detect and use the manually installed `protoc` if it exists.

```bash title="CLI"
rinf template
```

After running the command, you'll have new files and folders as your starter Rust template.

```diff title="Folder Tree"
    my_flutter_project/
    ├── android/
    ├── ios/
    ├── lib/
*   │   ├── main.dart
    │   └── ...
    ├── linux/
+   ├── messages/
+   │   ├── sample_folder/
+   │   ├── counter_number.proto
+   │   ├── fractal_art.proto
+   │   └── README.md
+   ├── native/
+   │   ├── hub/
+   │   │   ├── src/
+   │   │   └── Cargo.toml
+   │   ├── sample_crate/
+   │   │   ├── src/
+   │   │   └── Cargo.toml
+   │   └── README.md
    ├── web/
    ├── windows/
*   ├── .gitignore
+   ├── Cargo.toml
*   ├── pubspec.yaml
*   ├── README.md
    └── ...
```

Various comments are written in the actual code to help you understand the whole structure. Also, you might want to remove `sample_crate` in production.

If you already have a Rust crate that you want to use here, just put it inside `./native` and set it as a dependency of the `hub` crate.

Now by heading over to `./native/hub/src/lib.rs`, you can start writing Rust!

!!! info

    Rinf expects that the application's main logic is written in Rust, with Flutter solely serving the purpose of managing the GUI
