# 🆎 Rust-In-Flutter

Easily integrate Rust to make your Flutter app blazingly fast!

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

This high-level wrapper package simplifies Rust integration for your Flutter app without the need for code generation or native tooling. Designed with ease of use, future scalability, and exceptional performance in mind, it handles all the complicated aspects behind the scenes. Simply add this package to your Flutter project and you're ready to write Rust!

## Benefits

- Rust integration with the ability to use an arbitrary number of library crates
- RESTful API with easy request from Dart and response from Rust
- Async interaction with no blocking
- Streaming from Rust to Dart
- Restarting Rust logic on Dart's hot restart
- No memory copy when sending native data
- No complicated code generation during development
- No messing with sensitive native files in your project

## Platform Support

With this package, you don't have to start from scratch or face the challenging complexity of integrating Rust.

- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ Windows: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ⏸️ Web: Not now [but considered](https://github.com/cunarist/rust-in-flutter/issues/34)

> If you have any suggestions or want to report a bug, please leave it as an [issue](https://github.com/cunarist/rust-in-flutter/issues) or a [pull request](https://github.com/cunarist/rust-in-flutter/pulls). We will try to respond as quickly as possible.

# 👜 Installing Components

## Basic Steps

First, add this package to your Flutter project.

```bash
flutter pub add rust_in_flutter
```

Then install Rust toolchain. Refer to the [official Rust docs](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Extra Steps

If you are planning to compile your code for Linux, Windows, macOS, or iOS, you don't have to do anything more.

For Android, [install Android NDK](https://developer.android.com/studio/projects/install-ndk#specific-version). You must select the exact expected NDK version from your Flutter project, which can be seen in your `./android/app/build.gradle` file or [here](https://github.com/flutter/flutter/blob/stable/packages/flutter_tools/gradle/flutter.gradle). 

> Using extra build targets with Rust can sometimes present various issues. If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/rust-in-flutter/discussions) and open a Q&A thread for assistance.

# 👜 Applying Template

Simply run this in the command-line. Make sure that the current directory of the terminal is your Flutter project folder.

```bash
dart run rust_in_flutter:apply_template
```

Once you've run the command, there will be some new folders and files that will be your starter Rust template.

```diff
    my_flutter_project/
    ├── android/
    ├── ios/
    ├── lib/
    ├── linux/
+   ├── native/
+   │   ├── hub/
+   │   ├── sample_crate/
+   │   └── README.md
    ├── test/
    ├── web/
    ├── windows/
*   ├── .gitignore
+   ├── Cargo.toml
    ├── pubspec.yaml
    └── ...
```

Entry point of your Rust logic is the `hub` library crate. You might want to remove `sample_crate` in production.

Please keep in mind:
- Do NOT change the name of the `hub` crate or the `native` folder. Compilation presets expect the entry library crate to be located at `./native/hub`.
- Do NOT modify the `bridge` module inside `./native/hub/src`.
- You CAN name crates other than `hub` as you want.

Now by heading over to `./native/hub/src/lib.rs`, you can start writing Rust!

# 🧱 Tips

When requesting from Dart, you should specify the operation and address. This way of communication follows the definition of RESTful API.

```dart
import 'package:rust_in_flutter/rust_in_flutter.dart';
import 'package:msgpack_dart/msgpack_dart.dart';

void someFunction() async {
    var rustRequest = RustRequest(
      address: 'basicCategory.counterNumber',
      operation: Operation.Read,
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

Upon receiving requests from Rust, you should first classify them by address.

```rust
pub async fn handle_request(request_unique: RustRequestUnique) {
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

    let response_unique = RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    };
    respond_to_dart(response_unique);
}
```

Handling requests in Rust is as follows. Endpoint message schema is defined here because it will be different by address and operation type.

```rust
pub async fn calculate_something(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        Operation::Create => RustResponse::default(),
        Operation::Read => {
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
        Operation::Update => RustResponse::default(),
        Operation::Delete => RustResponse::default(),
    }
}
```

You can extend this RESTful API pattern and create hundreds and thousands of endpoints as you need. If you have a web background, this system might look familiar to you. More comments and details are included in the actual code inside the Rust template.

Ideally, **Flutter** would deal with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end can be completely separated, meaning that Dart and Rust codes are detachable from each other. These two worlds communicate through channels and streams.

Use [MessagePack](https://msgpack.org/) for serializing messages sent between Dart and Rust as provided by the Rust template, unless you have other reasons not to do so. For those who aren't familiar, MessagePack is a nested binary structure similar to JSON, but much faster and more efficient.

Data being sent between Dart and Rust are basically bytes arrays, represented as `Uint8List` in Dart and `Vec<u8>` in Rust. Though using MessagePack serialization is recommended, you can send any kind of bytes data as you wish, such as a high-resolution image or some kind of file data.

# ☕ Support Us

😉 If you are benefiting from the features of Rust-In-Flutter and find it helpful, why not consider supporting this project? Your generous donations contribute to the maintenance and development of Rust-In-Flutter, ensuring its continuous improvement and growth.

If you feel like so, please consider [buying us a coffee](https://www.buymeacoffee.com/cunarist).

# 🌋 Pioneers

This project was not done alone. There were various helpful projects that gave inspiration to the structure of this package. Credits to these wonderful efforts!

- https://github.com/fzyzcjy/flutter_rust_bridge
- https://github.com/superlistapp/super_native_extensions
- https://github.com/brickpop/flutter-rust-ffi
- https://github.com/corrosion-rs/corrosion
- https://github.com/irondash/cargokit