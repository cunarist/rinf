# Tutorial

> If you are using Rinf version 5 or earlier, please refer to the [historical docs](https://github.com/cunarist/rinf/blob/v5.4.0/documentation/docs/writing-code.md). With the introduction of Rinf version 6, a simpler way for communication between Dart and Rust has been implemented, and the system has undergone significant changes.

To grasp the core concepts, it's beneficial to follow a step-by-step tutorial. Detailed explanations will be provided in the upcoming sections, while the basics can be understood here.

## 🚨 From Dart to Rust

Let's say you want to create a new button in Dart that sends an array of numbers and a string to Rust. This signal is intended to notify that a user event has occurred, and it triggers Rust to perform some calculations on the data.

Write a new `.proto` file in the `./messages` directory representing the new Rust resource. Note that the message should have the comment `[RINF:DART-SIGNAL]` above it.

```proto
// messages/tutorial_resource.proto

syntax = "proto3";
package tutorial_resource;

// [RINF:DART-SIGNAL]
message MyNumberInput {
  repeated int32 input_numbers = 1;
  string input_string = 2;
}
```

Create a `Column` somewhere in your widget tree. This will contain our tutorial widgets.

```dart
// lib/main.dart
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [],
)
...
```

Next, generate Dart and Rust message code from `.proto` files.

```bash
rinf message
```

Create a button widget in Dart that accepts the user input.

```dart
// lib/main.dart
...
import 'package:example_app/messages/tutorial_resource.pb.dart';
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {
      myNumberInputSend(MyNumberInput( // GENERATED
          inputNumbers: [3, 4, 5],
          inputString: 'Zero-cost abstraction',
        ));
      },
      child: Text("Send a Signal from Dart to Rust"),
    ),
...
```

Let's listen to this message in Rust. This simple function will add one to each element in the array, capitalize all letters in the string, and return them.

```rust
// native/hub/src/sample_functions.rs
...
use crate::messages;
...
pub async fn listen_to_dart() {
    use messages::tutorial_resource::my_number_input_receiver; // GENERATED
    let mut receiver = my_number_input_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let my_input_number = dart_signal.message;

        let new_numbers: Vec<i32> = my_input_number
            .input_numbers
            .into_iter()
            .map(|x| x + 1)
            .collect();
        let new_string = my_input_number.input_string.to_uppercase();

        crate::debug_print!("{new_numbers:?}");
        crate::debug_print!("{new_string}");
    }
}
...
```

Make sure that this function is properly spawned.

```rust
// native/hub/src/lib.rs
...
async fn main() {
...
    tokio::spawn(sample_functions::listen_to_dart());
}
...
```

Now we can see the printed output in the command-line when clicking the button!

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

## 📡 From Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart.

Define the message. Note that the message should have the comment `[RINF:RUST-SIGNAL]` above it.

```proto
// messages/tutorial_resource.proto

syntax = "proto3";
package tutorial_resource;
...
// [RINF:RUST-SIGNAL]
message MyIncreasingNumber { int32 current_number = 1; }
```

Generate Dart and Rust message code from `.proto` files.

```bash
rinf message
```

Define an async Rust function that runs forever, sending numbers to Dart every second.

```rust
// native/hub/src/sample_functions.rs
...
use crate::messages;
...
pub async fn stream_increasing_number() {
    let mut current_number: i32 = 1;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        use messages::tutorial_resource::MyIncreasingNumber; // GENERATED
        let my_increasing_number = MyIncreasingNumber { current_number };

        use messages::tutorial_resource::my_increasing_number_send; // GENERATED
        my_increasing_number_send(my_increasing_number, None);

        current_number += 1;
    }
}
...
```

Spawn the async function in Rust.

```rust
// native/hub/src/lib.rs
...
async fn main() {
...
    tokio::spawn(sample_functions::stream_increasing_number());
}
...
```

Finally, receive the signals in Dart with `StreamBuilder` and rebuild the widget accordingly.

```dart
// lib/main.dart
...
import 'package:rinf/rinf.dart';
import 'package:example_app/messages/increasing_number.pb.dart'
    as increasingNumbers;
...
children: [
  StreamBuilder<RustSignal>(
    stream: myIncreasingNumberStream, // GENERATED
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text("Nothing received yet");
      }
      final message = rustSignal.message!;
      final currentNumber = message.currentNumber;
      return Text(currentNumber.toString());
    },
  ),
...
```

## 🤝 Back and Forth

Combine
