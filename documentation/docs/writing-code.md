# How to Write Code

## 📡 Tutorial

### Request from Dart, Response from Rust

Let's say that you want to make a new button that sends an array of numbers and a string from Dart to Rust to perform some calculation on it. You can follow these steps to understand how to send a request and wait for the response.

First, create a `Column` somewhere in your widget tree. This will contain our tutorial widgets.

```dart
// lib/main.dart
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [],
)
...
```

Create a new `.proto` file in `./messages` that represents the new Rust resource.

```proto
// messages/tutorial_resource.proto

syntax = "proto3";
package tutorial_resource;

message ReadRequest {
  repeated int32 input_numbers = 1;
  string input_string = 2;
}

message ReadResponse {
  repeated int32 output_numbers = 1;
  string output_string = 2;
}
```

Next, generate Dart and Rust message code from `.proto` files.

```bash
rinf message
```

Create a button widget in Dart that accepts the user input.

```dart
// lib/main.dart
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {},
      child: Text("Request to Rust"),
    ),
...
```

`onPressed` function should send a request to Rust. Let's create a `RustRequest` object.

```dart
// lib/main.dart
...
import 'package:rinf/rinf.dart';
import 'package:example_app/messages/tutorial_resource.pb.dart'
    as tutorialResource;
...
ElevatedButton(
  onPressed: () async {
    final requestMessage = tutorialResource.ReadRequest(
      inputNumbers: [3, 4, 5],
      inputString: 'Zero-cost abstraction',
    );
    final rustRequest = RustRequest(
      resource: tutorialResource.ID,
      operation: RustOperation.Read,
      message: requestMessage.writeToBuffer(),
    );
    final rustResponse = await requestToRust(rustRequest);
  },
  child: Text("Request to Rust"),
),
...
```

`requestToRust` function sends the request to Rust, returning a `RustResponse` object.

Now, write our new endpoint Rust function `sample_functions::handle_tutorial_resource`. This simple API endpoint will add one to each element in the array, capitalize all letters in the string, and return them.

```rust
// native/hub/src/sample_functions.rs
...
use crate::bridge::{RustOperation, RustRequest, RustResponse, RustSignal};
...
pub async fn handle_tutorial_resource(rust_request: RustRequest) -> RustResponse {
    use crate::messages::tutorial_resource::{ReadRequest, ReadResponse};

    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            let message_bytes = rust_request.message.unwrap();
            let request_message = ReadRequest::decode(message_bytes.as_slice()).unwrap();

            let new_numbers: Vec<i32> = request_message
                .input_numbers
                .into_iter()
                .map(|x| x + 1)
                .collect();
            let new_string = request_message.input_string.to_uppercase();

            let response_message = ReadResponse {
                output_numbers: new_numbers,
                output_string: new_string,
            };
            RustResponse {
                successful: true,
                message: Some(response_message.encode_to_vec()),
                blob: None,
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
...
```

The name of the new Rust resource was `tutorial_resource`. Make sure that the request handler function in Rust accepts this.

```rust
// native/hub/src/with_request.rs
...
use crate::bridge::{RustRequestUnique, RustResponse, RustResponseUnique};
use crate::messages;
use crate::sample_functions;
...
let rust_resource = rust_request.resource;
let rust_response = match rust_resource {
    messages::counter_number::ID => sample_functions::handle_counter_number(rust_request).await,
    messages::sample_folder::sample_resource::ID => {
        sample_functions::handle_sample_resource(rust_request).await
    }
    messages::sample_folder::deeper_folder::deeper_resource::ID => {
        sample_functions::handle_sample_resource(rust_request).await
    }
    messages::tutorial_resource::ID => {
        sample_functions::handle_tutorial_resource(rust_request).await // ADD THIS BLOCK
    }
    _ => RustResponse::default(),
};
...
```

Finally, when you receive a response from Rust in Dart, you can do anything with the bytes data in it.

```dart
// lib/main.dart
...
import 'package:rinf/rinf.dart';
import 'package:example_app/messages/tutorial_resource.pb.dart'
    as tutorialResource;
...
    final rustResponse = await requestToRust(rustRequest);
    final responseMessage =
        tutorialResource.ReadResponse.fromBuffer(
      rustResponse.message!,
    );
    print(responseMessage.outputNumbers);
    print(responseMessage.outputString);
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

We just simply print the message here, but the response data will be used for rebuilding Flutter widgets and updating states in real apps.

You can extend this RESTful API pattern and create hundreds and thousands of endpoints as you need. If you have a web background, this system might look familiar.

### Streaming from Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart. In this case, it would be inefficient for Dart to send requests repeatedly. This is where streaming is needed.

Let's start from our [default example](https://github.com/cunarist/rinf/tree/main/flutter_ffi_plugin/example).

Define the Rust resource and message schema.

```proto
// messages/increasing_number.proto

syntax = "proto3";
package increasing_number;

message StateSignal { int32 current_number = 1; }
```

Generate Dart and Rust message code from `.proto` files.

```bash
rinf message
```

Define an async Rust function that runs forever, sending numbers to Dart every second.

```rust
// native/hub/src/sample_functions.rs
...
use crate::bridge::{RustOperation, RustRequest, RustResponse, RustSignal};
use crate::bridge::send_rust_signal;
...
pub async fn stream_increasing_number() {
    use crate::messages::increasing_number::{StateSignal, ID};

    let mut current_number: i32 = 1;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let signal_message = StateSignal { current_number };
        let rust_signal = RustSignal {
            resource: ID,
            message: Some(signal_message.encode_to_vec()),
            blob: None,
        };
        send_rust_signal(rust_signal);

        current_number += 1;
    }
}
...
```

Spawn the async function in Rust.

```rust
// native/hub/src/lib.rs
...
mod sample_functions;
...
tokio::spawn(sample_functions::stream_mandelbrot());
tokio::spawn(sample_functions::stream_increasing_number()); // ADD THIS LINE
while let Some(request_unique) = request_receiver.recv().await {
...
```

Finally, receive the signals in Dart with `StreamBuilder`, filter them by resource with the `where` method, and rebuild the widget.

```dart
// lib/main.dart
...
import 'package:rinf/rinf.dart';
import 'package:example_app/messages/increasing_number.pb.dart'
    as increasingNumbers;
...
children: [
  StreamBuilder<RustSignal>(
    stream: rustBroadcaster.stream.where((rustSignal) {
      return rustSignal.resource == increasingNumbers.ID;
    }),
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text("Nothing received yet");
      } else {
        final singal = increasingNumbers.StateSignal.fromBuffer(
          rustSignal.message!,
        );
        final currentNumber = singal.currentNumber;
        return Text(currentNumber.toString());
      }
    },
  ),
...
```

We rebuild the widget with the received data here, but the streamed data can also be used to update Dart states in real apps.

## 🏷️ Message Details

### Meanings of Each Field

We've seen how to pass `RustRequest`, `RustResponse`, and `RustSignal` between Dart and Rust in this tutorial. Now let's go over to what exactly each field means.

- Field `resource`: This is an integer pointing to a virtual Rust resource that suits your app's design. Always provide `ID` of some message module generated by `rinf message`.

- Field `operation`: This accepts an enum value of `RustOperation` and can be one of create, read, update, and delete, since this system follows the definition of RESTful API.

- Field `message`: This is a bytes array created by Protobuf serialization. Note that it is not recommended to create Protobuf messages that are bigger than a few megabytes. To send large data, use `blob` instead. Sending bytes array is a zero-copy operation, though Protobuf serialization and deserialization process does involve memory copy. This field is optional and can be `null` or `None`.

- Field `blob`: This is also a bytes array intended to contain large data, possibly up to a few gigabytes. You can send any kind of binary as you wish such as a high-resolution image or some kind of file data. Sending a blob from Rust to Dart is a zero-copy operation, meaning there's no memory copy involved. In contrast, sending a blob from Dart to Rust is a copy operation. This field is optional and can be `null` or `None`.

### Response Timeout

By default, Dart will receive a failed `RustResponse` if Rust doesn't respond within 60 seconds. To set a custom timeout for your `RustRequest`, you can optionally provide a timeout argument to the `requestToRust` function like this:

```dart
final rustResponse = await requestToRust(
  rustRequest,
  timeout: const Duration(minutes: 5),
);
```

### Efficiency

While Rinf's API system may resemble that of web development, it relies only on native FFI for communication. It does NOT use any web protocols, hidden threads, and unnecessary memory copying to prevent any performance overhead.

## ⌛ Continuous Message Generation

If you add the optional argument `-w` or `--watch` to the `rinf message` command, the message code will automatically generated when `.proto` files are modified. When you add this argument, the command will not exit on its own.

```bash
rinf message --watch
```

## 🖨️ Printing for Debugging

You might be used to `println!` macro in Rust. However, using that macro isn't a very good idea in our apps made with Flutter and Rust because `println!` outputs cannot be seen on the web and mobile emulators.

When writing Rust code in the `hub` crate, you can simply print your debug message with the `debug_print!` macro provided by this framework like below. Once you use this macro, Flutter will take care of the rest.

```rust
crate::debug_print!("My object is {my_object:?}");
```

`debug_print!` is also better than `println!` because it only works in debug mode, resulting in a smaller and cleaner release binary.

## 🌅 Closing the App Gracefully

When the Flutter app is closed, the whole `tokio` runtime on the Rust side will be terminated automatically. However, some error messages can appear in the console if the Rust side sends messages to the Dart side after even after the Dart VM has stopped. To prevent this, you can call `Rinf.ensureFinalized()` in Dart to terminate all Rust tasks before closing the Flutter app.

```dart
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';

...

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final _appLifecycleListener = AppLifecycleListener(
    onExitRequested: () async {
      // Terminate Rust tasks before closing the Flutter app.
      await Rinf.ensureFinalized();
      return AppExitResponse.exit;
    },
  );

  @override
  void dispose() {
    _appLifecycleListener.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rinf Example',
      theme: ThemeData(
        useMaterial3: true,
        brightness: MediaQuery.platformBrightnessOf(context),
      ),
      home: MyHomePage(),
    );
  }
}

...
```
