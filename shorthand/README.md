# RIFS

[![Pub Version](https://img.shields.io/pub/v/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Popularity](https://img.shields.io/pub/popularity/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Points](https://img.shields.io/pub/points/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![GitHub Stars](https://img.shields.io/github/stars/cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/stargazers)
[![Build Test](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml/badge.svg)](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml?query=branch%3Amain)
[![GitHub License](https://img.shields.io/github/license/cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/blob/main/LICENSE)

**A command-line tool for simplifying the use of Rust-In-Flutter**

![Preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/ae82aad9-02f9-4a1e-93f9-69907511baf8)

This is a production-ready framework for creating beautiful and performant cross-platform apps using Flutter and Rust with batteries fully included. Simply add this framework to your app project, and you're all set to write Flutter and Rust together!

## 🎮 Demo

Visit the [demo](https://rif-demo.cunarist.com/) running on the web to experience the smoothness and delightfulness that comes from the combination of Flutter and Rust. You can also dive into the [example code](https://github.com/cunarist/rust-in-flutter/tree/main/example).

## 🖥️ Platform Support

All platforms available with Flutter are [tested](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml?query=branch%3Amain) and supported. Challenging build settings are automatically handled by this framework.

- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ Windows: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ✅ Web: Tested and supported

## 🎁 Benefits

- **Truly easy**: It only takes about about a minute or two to fully setup your app. No other solution provides this level of convenience.
- **Minimal**: This is not a bulky framework that requires you to install so many dependencies and use complicated CLI commands. Just focus on your code using your preferred Flutter and Rust libraries.
- **High-level interface**: No messing with sensitive build files, no concerns about memory safety. Stay with Dart and Rust that you're familiar with.
- **Low-level control**: Though the hard things are kept beneath the surface, you are free to modify the underlying logic such as concurrency or debugging features. There is no hidden mechanism that prevents your understanding.
- **Well maintained**: Our [automated workflows](https://github.com/cunarist/rust-in-flutter/actions) including build tests are always kept passing, thanks to the main branch protection rule. Also, the number of external dependencies is kept as low as possible and documentations are thoughtfully organized.
- **Efficient**: No memory copy when sending native data, no hidden threads and web workers with memory overhead. This is a really thin wrapper around Dart and Rust.
- **Scalable**: You are able to use an arbitrary number of Rust library crates as you want, perhaps including ones that you've already been working on.
- **Logical API**: You declare RESTful messages, not functions for APIs, which provides greater stability. Requesting from Dart and responsing from Rust, as well as streaming from Rust to Dart is possible. Messages are type-safe and flexible because it's powered by the well-known [Protobuf](https://protobuf.dev/) serialization. You also have the ability to send large binary data from Rust to Dart without any memory copy.
- **File-based messages**: No more headaches from extremely big API code files that hinders readability. Even defining hundereds and thousands of API endpoints between Dart and Rust is easy and clean.
- **Async interaction**: Rust operations will never block Flutter's main thread because they are spawned in a separate thread pool.
- **Convenient debugging**: All the debugging functionalities are provided by default, without the need for dealing with browsers or mobile emulators. Also, the whole Rust logic is automatically restarted on Dart's hot restart.
- **Reliable**: This framework simply provides a connection between Dart and Rust without complex code generation mechanism. Each component is backed by big communities, which is especially important for ensuring safety.

## 🐦 Why Use Flutter?

While Rust is a powerful language for high-performance native programming, its ecosystem for building graphical user interfaces is far from being mature. Though Rust already has some GUI frameworks, they don't compete with extensive support and smooth development experience that Flutter provides. It's only Flutter that compiles to all 6 major platforms from a single codebase.

Flutter is a powerful and versatile framework that has gained immense popularity for building cross-platform applications with stunning user interfaces. It provides declarative pattern, beautiful widgets, hot reload, convenient debugging tools, and dedicated packages for user interfaces right out-of-the-box.

## 🦀 Why Use Rust?

While Dart excels as an amazing object-oriented language for GUI apps, its non-native garbage collection may not always meet demanding performance requirements, and it may lack advanced data manipulation packages. This is where Rust steps in, offering an incredible speed advantage of roughly [2~40 times faster](https://programming-language-benchmarks.vercel.app/dart-vs-rust) than Dart, alongside the ability to leverage multiple threads and various crates that get the job done.

Rust has garnered a devoted following, being [the most loved programming language](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) on Stack Overflow. Its native performance, thanks to the zero-cost abstraction philosophy, ensures high productivity. Many developers foresee Rust potentially replacing C++ in the future. Rust's simplicity, memory safety, superior performance in various scenarios, vibrant community, and robust tooling support contribute to its growing popularity.

## 📖 Documentation

Check out the [documentation](https://rif-docs.cunarist.com) for everything you need to know about how to use this thing.

## 👥 Contributors

We appreciate your contribution to the development of this project! We're always open to discussions and pull requests, so please do not hesitate to leave your ideas or opinions at our GitHub repository.

[![GitHub contributors (via allcontributors.org)](https://contrib.rocks/image?repo=cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/graphs/contributors)
