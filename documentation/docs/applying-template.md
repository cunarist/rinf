# Applying Rust Template

> This section assumes that you've already created a Flutter project with `flutter create [my_app_name]`. You can get further guidance from [this awesome Flutter tutorial](https://docs.flutter.dev/get-started/codelab).

First of all, add this framework to your Flutter project.

```bash
flutter pub add rinf
```

Now install the command executable to easily run Rinf commands in the CLI.

```bash
cargo install rinf
```

Then, simply run this in the command-line from your Flutter project's directory.

```bash
rinf template
```

After running the command, you'll have new files and folders as your starter Rust template.

```diff
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

> If you're curious about all the available commands, use `rinf --help`.
