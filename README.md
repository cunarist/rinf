[English Guide](https://github.com/cunarist/rust-in-flutter/blob/main/README.md) · [한국어 설명서](https://github.com/cunarist/rust-in-flutter/blob/main/translations/KO.md) · [中文文档](https://github.com/cunarist/rust-in-flutter/blob/main/translations/ZH.md) · [日本語ガイド](https://github.com/cunarist/rust-in-flutter/blob/main/translations/JA.md)

# 🆎 Rust-In-Flutter

Easily integrate Rust to make your Flutter app blazingly fast!

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

Designed with ease of use, future scalability, and exceptional performance in mind, this lightweight framework handles all the complicated aspects behind the scenes. Simply add this package to your Flutter project and you're ready to write Rust!

## Benefits

- Rust integration with the ability to use an arbitrary number of library crates
- Being able to use an existing Rust crate as it is
- No messing with sensitive build files such as CMake, Gradle, Podfile, etc
- No complicated code generation during development
- Defining unlimited RESTful API endpoints without much effort
- Async interaction with easy request from Dart and response from Rust
- Streaming from Rust to Dart
- Restarting Rust logic on Dart's hot restart
- Minimal overhead
- No memory copy when sending native data

## Platform Support

All the challenging build settings are automatically handled by this package. Note that the files in your Flutter project are not affected.

- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ Windows: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ⏸️ Web: Not now [but considered](https://github.com/cunarist/rust-in-flutter/issues/34)

> If you have any suggestions or want to report a bug, please leave it as an [issue](https://github.com/cunarist/rust-in-flutter/issues) or a [pull request](https://github.com/cunarist/rust-in-flutter/pulls). We will try to respond as quickly as possible.

## Why Use Rust?

While Dart is an amazing object-oriented modern language for GUI apps, its performance sometimes does not meet the requirements because it's non-native garbage-collected language. That's where Rust comes into play. Rust's performance is known to be roughly about [2~40 times faster](https://programming-language-benchmarks.vercel.app/dart-vs-rust) than Dart, not to mention the ability to utilize multiple threads.

Rust is reported to be [the most loved programming language](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) on Stack Overflow, providing high productivity along with its native performance thanks to its zero-cast abstraction philosophy.

Of course, there are C and C++, but many developers believe that [Rust will eventually replace C++](https://dev.to/roy8/why-rust-will-replace-c-in-the-future-49op) as the go to programming language. There are many reasons why Rust is becoming more popular than C++. First of all, it’s much simpler and easier to learn. This is due to its design, which is focused on simplicity and memory safety. Additionally, Rust offers better performance than C++ in many cases. It also has a more active community and better tooling support.

You can read and learn Rust more [at the official book](https://doc.rust-lang.org/book/foreword.html).

# 👜 Installing Components

This section assumes that you've already installed [Flutter SDK](https://docs.flutter.dev/get-started/install) on your system and made a Flutter project with `flutter create` command. If you don't have a Flutter project yet, go ahead and make one following [this awesome tutorial](https://docs.flutter.dev/get-started/codelab).

First, add this package to your Flutter project.

```bash
flutter pub add rust_in_flutter
```

Then install Rust toolchain. Refer to the [official Rust docs](https://doc.rust-lang.org/book/ch01-01-installation.html).

Finally, check that your system is ready for compiling. You can repeat these commands to verify your system status after each installation step. If there are no issues in the output, you are good to go!

```bash
rustc --version
flutter doctor
```

## Build Tool Version Issues

- For Android apps, you should be using Rust 1.68 or higher due to [this issue](https://github.com/rust-lang/rust/pull/85806).
- For Android apps, variable `ndkVersion` in `./android/app/build.gradle` is necessary, but this might be missing if you've created your Flutter project with Flutter SDK 3.7 and earlier. Visit [this discussion](https://github.com/cunarist/rust-in-flutter/discussions/60) to solve this problem.

> Using various build targets with Rust can sometimes present various issues. If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/rust-in-flutter/discussions) and open a Q&A thread for assistance.

# 👜 Applying Template

Simply run this in the command-line from your Flutter project's directory.

```bash
dart run rust_in_flutter:apply_template
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
    use tokio::task::spawn;
    ...
    mod sample_functions;
    ...
    spawn(sample_functions::keep_drawing_mandelbrot());
+   spawn(sample_functions::keep_sending_numbers());
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
+           tokio::time::sleep(std::time::Duration::from_secs(1)).await;
+
+           #[derive(Serialize)]
+           struct RustSignalSchema {
+               current_number: i32,
+           }
+           let rust_signal = RustSignal {
+               address: String::from("sampleCategory.mandelbrot"),
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
```

# ✋ Tips

Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other. These two worlds communicate through streams.

Use MessagePack for serializing messages sent between Dart and Rust as provided by the Rust template unless you have other reasons not to do so. For those who aren't familiar, MessagePack is a nested binary structure similar to JSON, but faster and smaller. MessagePack also supports [more types](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system) of inner data compared to JSON, including binaries.

Data being sent between Dart and Rust are basically bytes arrays, represented as `Uint8List` in Dart and `Vec<u8>` in Rust. Though using MessagePack serialization is recommended, you can send any kind of bytes data as you wish such as a high-resolution image or some kind of file data, or just toss in a blank bytes array if you don't need additional details.

All build settings of Rust-In-Flutter ensures that all library files compiled from Rust crates are properly included in the final build, ready to be distributed. Therefore you do not need to worry about bundling library files.

Any other detailed announcements or informations are shared at [the discussions page](https://github.com/cunarist/rust-in-flutter/discussions). Please visit this page to read additional guides and ask questions.

# ☕ Support Us

😉 If you are benefiting from the features of Rust-In-Flutter and find it helpful, why not consider supporting this project? Your generous donations contribute to the maintenance and development of Rust-In-Flutter, ensuring its continuous improvement and growth.

If you feel like so, please consider [buying us a coffee](https://www.buymeacoffee.com/cunarist).
