# Introduction

## About Rinf

Rinf combines Flutter's UI ease with Rust's speed, enabling efficient, scalable cross-platform apps.

```{eval-rst}
.. raw:: html
   :file: _templates/icon_pair.html
```

### Flutter for Aesthetics

Flutter provides a mature and efficient ecosystem for building cross-platform graphical user interfaces. It compiles to all six major platforms and offers:

- Declarative UI pattern
- Beautiful widgets
- Hot reload for rapid development
- Dedicated debugging tools
- Comprehensive UI packages

### Rust for Performance

While Dart is excellent for UI, Rust excels in performance-critical tasks. Rust offers:

- 2-40x speed advantage over Dart
- Memory safety without garbage collection
- Multi-threading support
- Access to a vast ecosystem of crates
- A highly loved and robust community

### Benefits

- **Easy setup** – Setup takes only a minute or two.
- **Efficient** – Uses native FFI with no webviews, servers, or extra memory copying.
- **Minimal** – No bulky framework, just Flutter and Rust with minimal dependencies.
- **Event-driven** – Async-first, concurrency-oriented design.
- **Scalable** – Easily handle hundreds or thousands of message APIs.
- **High-level interface** – No need to modify build files or worry about memory safety.
- **Well maintained** – Automated tests, low dependencies, and clear documentation.
- **Convenient debugging** – No browsers or emulators; Rust logic restarts on hot restart.
- **Reliable** – Backed by strong communities and a simple, future-proof design.

## Cross-Platform Compatibility

Rinf enables seamless development across major platforms:

- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ Windows: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ✅ Web: Tested and supported

## Design

Rinf expects the application's main logic to be written in Rust, with Flutter solely managing the GUI.

Rather than relying on function calls, Rinf adopts a stream-based message-passing mechanism. This decouples the business logic from the UI, ensuring a clear separation of concerns.
