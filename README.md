# 🆎 About

Easily use Rust to make your Flutter app blazingly fast!

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

This high-level wrapper package simplifies Rust integration for your Flutter app without the need for code generation or native tooling. Designed with ease of use, future scalability, and exceptional performance in mind, it handles all the complicated aspects behind the scenes. Simply add this package to your Flutter project and you're ready to write Rust!

## Benefits

- Rust integration with the ability to use an arbitrary number of library crates
- RESTful API with easy request from Dart and response from Rust
- Async interaction with no blocking
- Streaming from Rust to Dart
- Restarting Rust logic on Dart's hot restart
- No memory copy when sending data
- No complicated code generation during development

## Platform Support

With this package, you don't have to start from scratch or face the challenging complexity of integrating Rust.

- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ Windows: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ⏸️ Web: Not now [but considered](https://github.com/cunarist/rust-in-flutter/issues/34)

> If you have any suggestions or want to report a bug, please leave it as an [issue](https://github.com/cunarist/rust-in-flutter/issues) or a [pull request](https://github.com/cunarist/rust-in-flutter/pulls). We will try to respond as quickly as possible.

# 🧱 Recommended Structure

Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other. These two worlds communicate through channels and streams.

Use [MessagePack](https://msgpack.org/) for serializing messages sent between Dart and Rust as provided by the template, if you have other reasons not to do so. For those who aren't familiar, MessagePack is a nested binary structure similar to JSON, but much faster and more efficient.

# 👜 Installing Rust Toolchain

Refer to the [Rust docs](https://doc.rust-lang.org/book/ch01-01-installation.html) to install Rust toolchain on your system. Because you're going to write Rust, only having the Flutter SDK on your system is not enough.

## Extra Steps

If you are planning to compile your code for Linux, Windows, macOS, or iOS, you don't have to do anything more.

For Android, [install Android NDK](https://developer.android.com/studio/projects/install-ndk#specific-version). You must select the exact expected NDK version from your Flutter project, which can be seen in your `./android/app/build.gradle` file or [here](https://github.com/flutter/flutter/blob/stable/packages/flutter_tools/gradle/flutter.gradle). 

> Using extra build targets with Rust can sometimes present various issues. If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/rust-in-flutter/discussions) and open a Q&A thread for assistance.
