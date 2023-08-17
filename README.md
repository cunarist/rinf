[English Guide](https://github.com/cunarist/rust-in-flutter/blob/main/README.md) · [한국어 설명서](https://github.com/cunarist/rust-in-flutter/blob/main/translations/KO.md) · [中文文档](https://github.com/cunarist/rust-in-flutter/blob/main/translations/ZH.md) · [日本語ガイド](https://github.com/cunarist/rust-in-flutter/blob/main/translations/JA.md)

[![Pub Version](https://img.shields.io/pub/v/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Popularity](https://img.shields.io/pub/popularity/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Points](https://img.shields.io/pub/points/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![GitHub Stars](https://img.shields.io/github/stars/cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/stargazers)
[![Build Test](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml/badge.svg)](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml?query=branch%3Amain)
[![GitHub License](https://img.shields.io/github/license/cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/blob/main/LICENSE)

# 🆎 Rust-In-Flutter

Easily integrate Rust to make your Flutter app blazingly fast!

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

No messing with sensitive build files, no complicated code generation during development.

Designed for ease of use, future scalability, and unparalleled performance, this lightweight framework takes care of all the complexity behind the scenes. Simply add this package to your Flutter project, and you're all set to write Rust!

## Platform Support

All platforms available with Flutter are [tested](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml?query=branch%3Amain) and supported. Also, challenging build settings are automatically handled by this package.

| Dev OS  | Linux | Android | Windows | macOS | iOS | Web |
| ------- | ----- | ------- | ------- | ----- | --- | --- |
| Linux   | ✅    | ✅      | -       | -     | -   | ✅  |
| Windows | -     | ✅      | ✅      | -     | -   | ✅  |
| macOS   | -     | ✅      | -       | ✅    | ✅  | ✅  |

## Benefits

- Rust integration with the ability to use an arbitrary number of library crates
- Async interaction with no blocking
- RESTful API with easy request from Dart and response from Rust
- Streaming from Rust to Dart
- Automatic restart of Rust logic on Dart's hot restart
- No memory copy when sending native data

## Why Use Rust?

While Dart excels as an amazing object-oriented language for GUI apps, its non-native garbage collection may not always meet demanding performance requirements. This is where Rust steps in, offering an incredible speed advantage of roughly [2~40 times faster](https://programming-language-benchmarks.vercel.app/dart-vs-rust) than Dart, alongside the ability to leverage multiple threads.

Rust has garnered a devoted following, being [the most loved programming language](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) on Stack Overflow. Its native performance, thanks to the zero-cost abstraction philosophy, ensures high productivity. Many developers foresee Rust potentially replacing C++ in the future. Rust's simplicity, memory safety, superior performance in various scenarios, vibrant community, and robust tooling support contribute to its growing popularity.

To delve deeper into the world of Rust, check out the official book: [https://doc.rust-lang.org/book/foreword.html](https://doc.rust-lang.org/book/foreword.html).

# 🛠️ Installing Rust Toolchain

> This section assumes that [Flutter SDK](https://docs.flutter.dev/get-started/install) is installed on your system.

Installing Rust toolchain is very easy. Just head over to the [official installation page](https://www.rust-lang.org/tools/install) and follow the instructions.

Once Rust toolchain installation is completed, check that your system is ready. Flutter SDK might require some additional components to target various platforms. If there are no issues in the output, you are good to go!

```bash
rustc --version
flutter doctor
```

# 👜 Applying Rust Template

> This section assumes that you've already created a Flutter project. If you haven't, go ahead and make one following [this awesome official tutorial](https://docs.flutter.dev/get-started/codelab).

First of all, add this package to your Flutter project.

```bash
flutter pub add rust_in_flutter
```

Then, simply run this in the command-line from your Flutter project's directory.

```bash
dart run rust_in_flutter template
```

Once you've run the command, there will be some new files and folders that will be your starter Rust template.

```diff
    my_flutter_project/
    ├── android/
    ├── ios/
    ├── lib/
*   │   ├── main.dart
    │   └── ...
    ├── linux/
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
    └── ...
```

Don't forget to read the `./native/README.md` file first. Various comments are written in the code to help you understand the structure. Also, you might want to remove `sample_crate` in production. If you already have a Rust crate that you want to use here, just put it inside `./native` and set it as a dependency of the `hub` crate.

Now by heading over to `./native/hub/src/lib.rs`, you can start writing Rust!

# 👟 Running and Building

## For Native Platforms

The following commands can be used to run and build Flutter apps for native platforms.

To run the app:

```bash
flutter run
```

To build the app for a specific platform:

```bash
flutter build (platform) # Replace it with a platform name
```

## For the Web

You need to manually build webassembly module from Rust before running or building the app for the web.

To serve the web application:

```bash
dart run rust_in_flutter wasm
flutter run --profile  # Choose a browser
```

To build the optimized release version of the web application:

```bash
dart run rust_in_flutter wasm --release
flutter build web
```

# 🧱 How to Write Code

## Request from Dart, Response from Rust

As your app grows bigger, you will need to define new Rust API endpoints.

Let's say that you want to make a new button that sends an array of numbers and a string from Dart to Rust to perform some calculation on it. You can follow these steps to understand how to send a request and wait for the response.

Let's start from our [default example](https://github.com/cunarist/rust-in-flutter/tree/main/example). Create a button widget in Dart that will accept the user input.

```diff
  // lib/main.dart
  ...
  child: Column(
    mainAxisAlignment: MainAxisAlignment.center,
    children: [
+     ElevatedButton(
+       onPressed: () async {},
+       child: Text("Request to Rust"),
+     ),
  ...
```

`onPressed` function should send a request to Rust. Let's create a `RustRequest` object first.

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  ElevatedButton(
+   onPressed: () async {
+     final rustRequest = RustRequest(
+       address: 'myCategory.someData',
+       operation: RustOperation.Read,
+       bytes: serialize(
+         {
+           'input_numbers': [3, 4, 5],
+           'input_string': 'Zero-cost abstraction',
+         },
+       ),
+     );
+   },
    child: Text("Request to Rust"),
  ),
  ...
```

`address` can be any string that suits your app's design, represented as camelcase strings layered by dots. `operation` can be one of create, read, update, and delete, since this system follows the definition of RESTful API. As the name suggests, `bytes` is just a simple bytes array, usually created by [MessagePack](https://msgpack.org/) serialization.

Now we should send this request to Rust. `requestToRust` function does this job, which returns a `RustResponse` object.

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  ElevatedButton(
    onPressed: () async {
      final rustRequest = RustRequest(
        address: 'myCategory.someData',
        operation: RustOperation.Read,
        bytes: serialize(
          {
            'input_numbers': [3, 4, 5],
            'input_string': 'Zero-cost abstraction',
          },
        ),
      );
+     final rustResponse = await requestToRust(rustRequest);
    },
    child: Text("Request to Rust"),
  ),
    ...
```

So, our new API address is `myCategory.someData`. Make sure that the request handler function in Rust accepts this.

```diff
    // native/hub/src/with_request.rs
    ...
    use crate::bridge::api::RustResponse;
    use crate::sample_functions;
    ...
    let layered: Vec<&str> = rust_request.address.split('.').collect();
    let rust_response = if layered.is_empty() {
        RustResponse::default()
    } else if layered[0] == "basicCategory" {
        if layered.len() == 1 {
            RustResponse::default()
        } else if layered[1] == "counterNumber" {
            sample_functions::calculate_something(rust_request).await
        } else {
            RustResponse::default()
        }
+   } else if layered[0] == "myCategory" {
+       if layered.len() == 1 {
+           RustResponse::default()
+       } else if layered[1] == "someData" {
+           sample_functions::some_data(rust_request).await
+       } else {
+           RustResponse::default()
+       }
    } else {
        RustResponse::default()
    };
    ...
```

This `sample_functions::some_data` is our new endpoint Rust function. This simple API endpoint will add one to each element in the array, capitalize all letters in the string, and return them. Message schema is defined in the match statement because it will be different by the operation type.

```diff
    // native/hub/src/sample_functions.rs
    ...
    use crate::bridge::api::RustOperation;
    use crate::bridge::api::RustRequest;
    use crate::bridge::api::RustResponse;
    use rmp_serde::from_slice;
    use rmp_serde::to_vec_named;
    use serde::Deserialize;
    use serde::Serialize;
    ...
+   pub async fn some_data(rust_request: RustRequest) -> RustResponse {
+       match rust_request.operation {
+           RustOperation::Create => RustResponse::default(),
+           RustOperation::Read => {
+               #[allow(dead_code)]
+               #[derive(Deserialize)]
+               struct RustRequestSchema {
+                   input_numbers: Vec<i8>,
+                   input_string: String,
+               }
+               let slice = rust_request.bytes.as_slice();
+               let received: RustRequestSchema = from_slice(slice).unwrap();
+
+               let new_numbers = received.input_numbers.into_iter().map(|x| x + 1).collect();
+               let new_string = received.input_string.to_uppercase();
+
+               #[derive(Serialize)]
+               struct RustResponseSchema {
+                   output_numbers: Vec<i8>,
+                   output_string: String,
+               }
+               RustResponse {
+                   successful: true,
+                   bytes: to_vec_named(&RustResponseSchema {
+                       output_numbers: new_numbers,
+                       output_string: new_string,
+                   })
+                   .unwrap(),
+               }
+           }
+           RustOperation::Update => RustResponse::default(),
+           RustOperation::Delete => RustResponse::default(),
+       }
+   }
    ...
```

Finally, when you receive a response from Rust in Dart, you can do anything with the bytes data in it.

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  ElevatedButton(
    onPressed: () async {
      final rustRequest = RustRequest(
        address: 'myCategory.someData',
        operation: RustOperation.Read,
        bytes: serialize(
          {
            'input_numbers': [3, 4, 5],
            'input_string': 'Zero-cost abstraction',
          },
        ),
      );
      final rustResponse = await requestToRust(rustRequest);
+     final message = deserialize(rustResponse.bytes) as Map;
+     print(message["output_numbers"]);
+     print(message["output_string"]);
    },
    child: Text("Request to Rust"),
  ),
    ...
```

And we can see the printed output in the command-line!

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

We just simply print the message here, but the response data will be used for rebuilding Flutter widgets in real apps.

You can extend this RESTful API pattern and create hundreds and thousands of endpoints as you need. If you have a web background, this system might look familiar.

## Streaming from Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart. In this case, it would be inefficient for Dart to send requests repeatedly. This is where streaming is needed.

Let's start from our [default example](https://github.com/cunarist/rust-in-flutter/tree/main/example). Spawn an async function in Rust.

```diff
    // native/hub/src/lib.rs
    ...
    mod sample_functions;
    ...
    crate::spawn(sample_functions::keep_drawing_mandelbrot());
+   crate::spawn(sample_functions::keep_sending_numbers());
    while let Some(request_unique) = request_receiver.recv().await {
    ...
```

Define the async Rust function that runs forever, sending numbers to Dart every second.

```diff
    // native/hub/src/sample_functions.rs
    ...
    use crate::bridge::api::RustSignal;
    use crate::bridge::send_rust_signal;
    ...
    use rmp_serde::to_vec_named;
    ...
    use serde::Serialize;
    ...
+   pub async fn keep_sending_numbers() {
+       let mut current_number: i32 = 1;
+       loop {
+           crate::time::sleep(std::time::Duration::from_secs(1)).await;
+
+           #[derive(Serialize)]
+           struct RustSignalSchema {
+               current_number: i32,
+           }
+           let rust_signal = RustSignal {
+               address: String::from("myCategory.increasingNumbers"),
+               bytes: to_vec_named(&RustSignalSchema {
+                   current_number: current_number,
+               })
+               .unwrap(),
+           };
+           send_rust_signal(rust_signal);
+           current_number += 1;
+       }
+   }
    ...
```

Finally, receive the signals in Dart with `StreamBuilder`, filter them by address with the `where` method, and rebuild the widget.

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  children: [
+   StreamBuilder<RustSignal>(
+     stream: rustBroadcaster.stream.where((rustSignal) {
+       return rustSignal.address == "myCategory.increasingNumbers";
+     }),
+     builder: (context, snapshot) {
+       final received = snapshot.data;
+       if (received == null) {
+         return Text("Nothing received yet");
+       } else {
+         final singal = deserialize(received.bytes) as Map;
+         final currentNumber = singal["current_number"] as int;
+         return Text(currentNumber.toString());
+       }
+     },
+   ),
  ...
```

# ✋ FAQ

**Q**. When should I use Rust?

**A**. Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other.

**Q**. How are data passed between Dart and Rust?

**A**. Data being sent between Dart and Rust are basically bytes arrays, represented as `Uint8List` in Dart and `Vec<u8>` in Rust. Though using MessagePack serialization is recommended, you can send any kind of bytes data as you wish such as a high-resolution image or some kind of file data, or just toss in a blank bytes array if you don't need additional details.

**Q**. What is "MessagePack" and why is it recommended?

**A**. MessagePack is a nested binary structure similar to JSON, but faster and smaller. MessagePack also supports [more types](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system) of inner data compared to JSON, including binaries. Use MessagePack for serializing messages sent between Dart and Rust as provided by the Rust template unless you have other reasons not to do so.

**Q**. Where are the library files generated from Rust crates?

**A**. All build settings of Rust-In-Flutter ensures that all library files compiled from Rust crates are properly included in the final build, ready to be distributed. Therefore you do not need to worry about bundling library files.

**Q**. Android app build has failed. What should I do?

**A**. For Android apps, you should be using Rust 1.68 or higher due to [this issue](https://github.com/rust-lang/rust/pull/85806). Also, variable `ndkVersion` should be present in `./android/app/build.gradle` file, but it may be missing if you've created your Flutter project with Flutter SDK 3.7 and earlier. Visit [this discussion](https://github.com/cunarist/rust-in-flutter/discussions/60) to solve this problem.

**Q**. Where should I ask for help?

**A**. If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/rust-in-flutter/discussions) and open a Q&A thread for assistance. Please visit this page to read additional guides and ask questions.

**Q**. How does concurrency work under the hood?

**A**. On native platforms, Dart runs in a single thread as usual, while Rust utilizes the async `tokio` runtime to take advantage of all cores on the computer, allowing async tasks to run efficiently within that runtime. On the web, Dart still runs in the main thread, but Rust operates only within a single web worker (thread). This is a necessary constraint because web workers do not share memory, but it is still possible for Rust to perform concurrent operations within that one dedicated thread by converting Rust `Future`s into JavaScript `Promise`s and passing them into the JavaScript event loop.

**Q**. How do I make messages completely type-safe?

**A**. When using MessagePack serialization, IDEs like VSCode might not provide full intellisense support, leading to potential type-related issues. To ensure proper type checking in IDEs, you can choose Protobuf instead of MessagePack. Protobuf is a type-safe serialization method that can help prevent data type-related errors. Although integrating Protobuf is beyond the scope of this package, you can refer to its [official documentation](https://protobuf.dev/) for implementation details.

**Q**. The built web version shows errors related to cross-origin policy in the browser console.

**A**. After building your binary and preparing it for deployment, ensure your web server is configured to include cross-origin related HTTP headers in its responses. Set `cross-origin-opener-policy` to `same-origin`, `cross-origin-embedder-policy` to `require-corp` or `credentialless`. These headers enable clients using your website to gain access `SharedArrayBuffer` web API, which is needed by this framework. `SharedArrayBuffer` is something similar to shared memory on the web.

**Q**. Will changes made to Rust code take effect upon Dart's hot restart?

**A**. No, the updated Rust code cannot be loaded upon Dart's hot restart. To incorporate the changes, the app needs to be re-compiled, as the app binary must be linked to the newly compiled Rust library files again. This limitation arises from the Rust compilation process, as Rust does not inherently support a hot restart feature. Still, Dart's hot restart does restart the Rust logic, in other words, the `main()` function.

# 🌟 Contributors

We appreciate your contribution to the development of this project!

[![GitHub contributors (via allcontributors.org)](https://contrib.rocks/image?repo=cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/graphs/contributors)

# 👏 Thanks

These are tools for connecting Flutter and native compilers.

Rust-In-Flutter was not made alone. There were pioneers that gave inspiration to the structure of this package. Credits to these wonderful efforts!

- https://github.com/fzyzcjy/flutter_rust_bridge
- https://github.com/superlistapp/super_native_extensions
- https://github.com/brickpop/flutter-rust-ffi
- https://github.com/corrosion-rs/corrosion
- https://github.com/irondash/cargokit

# ☕ Support Us

If you are benefiting from the features of Rust-In-Flutter and find it helpful, why not consider supporting this project? Your generous donations contribute to the maintenance and development of Rust-In-Flutter, ensuring its continuous improvement and growth. 😉

If you feel like so, please consider [buying us a coffee](https://www.buymeacoffee.com/cunarist).
