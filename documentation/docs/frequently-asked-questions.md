# FAQ

### Where should I ask for help?

If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/rinf/discussions) and open a Q&A thread for assistance.

### Where should I use Rust?

Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other.

### How are data passed between Dart and Rust?

Data being sent between Dart and Rust are basically bytes arrays, represented as `Uint8List` in Dart and `Vec<u8>` in Rust. You can send Protobuf messages and any binary data as you wish such as a high-resolution image or some kind of file data. If you don't need any additional details, just toss in `None`.

### Where are the library files generated from Rust crates?

All build settings of Rinf ensures that all library files compiled from Rust crates are properly included in the final build, ready to be distributed. Therefore you do not need to worry about bundling library files.

### Android app build has failed. What should I do?

For Android apps, you should be using Rust 1.68 or higher due to [this issue](https://github.com/rust-lang/rust/pull/85806).

Also, The NDK version that your project expects is specified in `./android/app/build.gradle` file as `ndkVersion` variable inside `android` block. The value of this `ndkVersion` should be `flutter.ndkVersion` and you should be using Flutter SDK [3.10 or higher](https://docs.flutter.dev/release/release-notes/release-notes-3.10.0). However, `ndkVersion` can be absent if you've created your Flutter project with Flutter SDK 3.7 and earlier. If `ndkVersion` is not defined in your `./android/app/build.gradle` file, go ahead and write one yourself.

Add that line to `./android/app/build.gradle` file:

```gradle
..
android {
    namespace "com.cunarist.rinf_example"
    compileSdkVersion flutter.compileSdkVersion
    ndkVersion flutter.ndkVersion // <------ THIS LINE

    compileOptions {
        sourceCompatibility JavaVersion.VERSION_1_8
        targetCompatibility JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = '1.8'
    }

    sourceSets {
        main.java.srcDirs += 'src/main/kotlin'
    }
..
```

### How does concurrency work under the hood?

On native platforms, Dart runs in a single thread as usual, while Rust utilizes the async `tokio` runtime to take advantage of all cores on the computer, allowing async tasks to run efficiently within that runtime. On the web, Dart still runs in the main thread, but Rust operates only within a single web worker (thread). This is a necessary constraint because web workers do not share memory, but it is still possible for Rust to perform concurrent operations within that one dedicated thread by converting Rust `Future`s into JavaScript `Promise`s and passing them into the JavaScript event loop.

### The built web version shows errors related to cross-origin policy in the browser console.

After building your binary and preparing it for deployment, ensure that your web server is configured to include cross-origin-related HTTP headers in its responses. Set the [`cross-origin-opener-policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy) to `same-origin` and [`cross-origin-embedder-policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy) to `require-corp`. These headers enable clients using your website to gain access to `SharedArrayBuffer` web API, which is something similar to shared memory on the web. Additionally, don't forget to specify the MIME type `application/wasm` for `.wasm` files within the server configurations to ensure optimal performance.

### Will changes made to Rust code take effect upon Dart's hot restart?

No, the updated Rust code cannot be loaded upon Dart's hot restart. To incorporate the changes, the app needs to be re-compiled, as the app binary must be linked to the newly compiled Rust library files again. This limitation arises from the Rust compilation process, as Rust does not inherently support a hot restart feature. Still, Dart's hot restart does restart the Rust logic, in other words, the `main()` function.

### How do I use nightly Rust?

In order to use nightly Rust, you need to add a cargokit configuration file. Cargokit is the build connector between Dart and Rust used by this framework.

Create that `./native/hub/cargokit.yaml` file:

```yaml
cargo:
  release:
    toolchain: nightly
```

### Is it safe enough to pass secret parameters between Dart and Rust?

It is completely safe to pass secret parameters between Dart and Rust. Some other Rust GUI frameworks use HTTP or websockets to communicate between GUI and Rust, which is quite insecure. However, that's not the case for Rinf because messages are passed within the Flutter app's process. This is not only secure but also performant because message passing is a zero-copy operation on the memory. Please note that while it is hard to reverse-engineer compiled native binaries to search for secret keys or params, it is generally not recommended to hardcode sensitive information in the application itself. This caution applies to this framework as well as any other distributed binaries.

### How do I use well-known types in Protobuf?

When using well-known Protobuf types like `Timestamp` in your Dart project, it may be necessary to compile them manually if they are not automatically generated by the `rinf message` command. Here's a step-by-step guide on how to do it:

#### 1. Create a Proto File

Start by creating a `.proto` descriptor file that contains the definition of the well-known type you want to use. For example, if you want to use the `Timestamp` type, your proto file might look like this:

```protobuf
syntax = "proto3";
package my_resource_name;

import "google/protobuf/timestamp.proto";

// Your custom messages here
```

Ensure that you import the necessary well-known type(s) at the beginning of your file, as shown in the example.

#### 2. Compile to Dart

Open your terminal and navigate to the root directory of your Dart project. Then, use the `protoc` command to compile the well-known type to Dart. For example, to compile the `Timestamp` type, you can run the following command:

```shell
protoc --dart_out=./lib/messages google/protobuf/timestamp.proto
```

Be sure to compile to `./lib/messages` because `rinf message` compiles everything into there.

#### 3. Integrate Generated Files

After running the `protoc` command, you'll find the generated Dart files in the specified output directory. You can now integrate these generated files into your Dart project like any other Dart module.

```dart
// In your Dart code, you can import and use the well-known type:
import 'package:my_app/messages/google/protobuf/timestamp.pb.dart';
```

### Can I use this in pure Dart projects?

No, this framework only supports GUI Flutter apps because it's basically a 'Flutter FFI plugin'. This framework does NOT support other types of projects:

- Flutter GUI app: **Mainly supported ☀️**
- Flutter plugin: Not supported, but this package can be an inspiration
- Dart CLI app: Might not work, this framework has to hook onto the Flutter SDK
- Dart package: Not supported, unable to hook onto Dart SDK builds

However, we also promise to deliver the best development experience as it is mainly focused on this **one** category.

### What happens when a panic occurs in Rust?

A Rust panic doesn't crash the app; it simply cancels the spawned async task. You don't need to worry about app stability due to Rust panics. When a panic occurs, the information will be displayed in the CLI if the app is running in debug mode.

### How do I make Rust-analyzer lint in webassembly mode?

There might be various Rust codes with these attribute above:

```rust
#[cfg(target_family = "wasm")]
...
#[cfg(not(target_family = "wasm"))]
...
```

Since the environments of the web and native platforms are so different, there are times when you need to use these attributes to include and exclude parts of the code depending on whether they are targeting web or not.

By default, Rust-analyzer runs in native mode. To make it run in webassembly mode, create that `.cargo/config.toml` file:

```toml
[build]
# Uncomment the line below to switch Rust-analyzer to perform
# type checking and linting in webassembly mode, for the web target.
# You might have to restart Rust-analyzer for this change to take effect.
target = "wasm32-unknown-unknown"
rustflags = ["-C", "target-feature=+atomics,+bulk-memory,+mutable-globals"]

[unstable]
build-std = ['std', 'panic_abort']
```
