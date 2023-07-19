[English Guide](https://github.com/cunarist/rust-in-flutter/blob/main/README.md) · [中文文档](https://github.com/cunarist/rust-in-flutter/blob/main/translations/ZH.md) · [한국어 설명서](https://github.com/cunarist/rust-in-flutter/blob/main/translations/KO.md)

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

While Dart is an amazing object-oriented modern language, its performance sometimes does not meet the requirements because it's non-native garbage-collected language. That's where Rust comes into play. Rust's performance is known to be roughly about [2~40 times faster](https://programming-language-benchmarks.vercel.app/dart-vs-rust) than Dart, not to mention the ability to utilize multiple threads.

# 👜 Installing Components

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

Simply run this in the command-line from the Flutter project folder.

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

Don't forget to read the `./native/README.md` file first. Also, you might want to remove `sample_crate` in production. If you already have a Rust crate that you want to use here, just put it inside `./native` and set it as a dependency of the `hub` crate.

Now by heading over to `./native/hub/src/lib.rs`, you can start writing Rust!

# 🧱 Tips

When requesting from Dart, you should specify the operation and address. This way of communication follows the definition of RESTful API.

```dart
import 'package:msgpack_dart/msgpack_dart.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';

void someFunction() async {
    var rustRequest = RustRequest(
      address: 'basicCategory.counterNumber',
      operation: RustOperation.Read,
      bytes: serialize(
        {
          'letter': 'Hello from Dart!',
          'before_number': 888,
          'dummy_one': 1,
          'dummy_two': 2,
          'dummy_three': [3, 4, 5]
        },
      ),
    );

    var rustResponse = await requestToRust(rustRequest);
    var message = deserialize(rustResponse.bytes) as Map;
    var innerValue = message['after_number'] as int;
}
```

Upon receiving the request in Rust, you should first classify them by address.

```rust
pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

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
    } else {
        RustResponse::default()
    };

    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}
```

Endpoint function in Rust would be like this. Message schema is defined in the match statement because it will be different by the operation type.

```rust
pub async fn calculate_something(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            #[allow(dead_code)]
            #[derive(Deserialize)]
            struct RustRequestSchema {
                letter: String,
                before_number: i32,
                dummy_one: i32,
                dummy_two: i32,
                dummy_three: Vec<i32>,
            }
            let slice = rust_request.bytes.as_slice();
            let received: RustRequestSchema = from_slice(slice).unwrap();
            println!("{:?}", received.letter);

            let before_value = received.before_number;
            let after_value = sample_crate::add_seven(before_value);

            #[derive(Serialize)]
            struct RustResponseSchema {
                after_number: i32,
                dummy_one: i32,
                dummy_two: i32,
                dummy_three: Vec<i32>,
            }
            RustResponse {
                successful: true,
                bytes: to_vec_named(&RustResponseSchema {
                    after_number: after_value,
                    dummy_one: 1,
                    dummy_two: 2,
                    dummy_three: vec![3, 4, 5],
                })
                .unwrap(),
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
```

You can extend this RESTful API pattern and create hundreds and thousands of endpoints as you need. If you have a web background, this system might look familiar. More comments and details are included in the actual code inside the Rust template.

Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other. These two worlds communicate through channels and streams.

Use [MessagePack](https://msgpack.org/) for serializing messages sent between Dart and Rust as provided by the Rust template unless you have other reasons not to do so. For those who aren't familiar, MessagePack is a nested binary structure similar to JSON, but faster and smaller.

Data being sent between Dart and Rust are basically bytes arrays, represented as `Uint8List` in Dart and `Vec<u8>` in Rust. Though using MessagePack serialization is recommended, you can send any kind of bytes data as you wish such as a high-resolution image or some kind of file data, or just toss in a blank bytes array if you don't need additional details.

The library files compiled from Rust crates are automatically included in the build folder or the executable binary. No manual bundling is needed.

# ☕ Support Us

😉 If you are benefiting from the features of Rust-In-Flutter and find it helpful, why not consider supporting this project? Your generous donations contribute to the maintenance and development of Rust-In-Flutter, ensuring its continuous improvement and growth.

If you feel like so, please consider [buying us a coffee](https://www.buymeacoffee.com/cunarist).
