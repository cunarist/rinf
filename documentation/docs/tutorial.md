# Basic Tutorial

To grasp the basic concepts, it's beneficial to follow a step-by-step tutorial.

Before we start, make sure that there's a `Column` somewhere in your widget tree. This will contain our tutorial widgets.

```dart title="lib/main.dart"
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [],
)
...
```

## 🚨 From Dart to Rust

Let's say that you want to create a new button in Dart that sends an array of numbers and a string to Rust. We need a signal to notify Rust that a user event has occurred.

Write a new `.proto` file in the `./messages` directory with a new message. Note that the message should have the comment `[RINF:DART-SIGNAL]` above it.

```proto title="messages/tutorial_resource.proto"
syntax = "proto3";
package tutorial_resource;

// [RINF:DART-SIGNAL]
message MyPreciousData {
  repeated int32 input_numbers = 1;
  string input_string = 2;
}
```

Next, generate Dart and Rust message code from `.proto` files.

```bash title="CLI"
rinf message
```

Create a button widget in Dart that accepts the user input.

```dart title="lib/main.dart"
...
import 'package:example_app/messages/tutorial_resource.pb.dart';
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {
        MyPreciousData(
          inputNumbers: [3, 4, 5],
          inputString: 'Zero-cost abstraction',
        ).sendSignalToRust(null); // GENERATED
      },
      child: Text("Send a Signal from Dart to Rust"),
    ),
...
```

Let's listen to this message in Rust. This simple function will add one to each element in the array and capitalize all letters in the string.

```rust title="native/hub/src/sample_functions.rs"
...
use crate::debug_print;
use crate::messages;
...
pub async fn calculate_precious_data() {
    use messages::tutorial_resource::*;

    let mut receiver = MyPreciousData::get_dart_signal_receiver(); // GENERATED
    while let Some(dart_signal) = receiver.recv().await {
        let my_precious_data = dart_signal.message;

        let new_numbers: Vec<i32> = my_precious_data
            .input_numbers
            .into_iter()
            .map(|x| x + 1)
            .collect();
        let new_string = my_precious_data.input_string.to_uppercase();

        debug_print!("{new_numbers:?}");
        debug_print!("{new_string}");
    }
}
...
```

```rust title="native/hub/src/lib.rs"
...
mod sample_functions;
...
async fn main() {
...
    tokio::spawn(sample_functions::calculate_precious_data());
}
```

Now we can see the printed output in the command-line when clicking the button!

```title="Output"
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

## 📡 From Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart.

Define the message. Note that the message should have the comment `[RINF:RUST-SIGNAL]` above it.

```proto title="messages/tutorial_resource.proto"
syntax = "proto3";
package tutorial_resource;
...
// [RINF:RUST-SIGNAL]
message MyAmazingNumber { int32 current_number = 1; }
```

Generate Dart and Rust message code from `.proto` files.

```bash title="CLI"
rinf message
```

Define an async Rust function that runs forever, sending numbers to Dart every second.

```rust title="native/hub/src/sample_functions.rs"
...
use crate::messages;
...
pub async fn stream_amazing_number() {
    use messages::tutorial_resource::*;

    let mut current_number: i32 = 1;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        MyAmazingNumber { current_number }.send_signal_to_dart(None); // GENERATED
        current_number += 1;
    }
}
...
```

```rust title="native/hub/src/lib.rs"
...
mod sample_functions;
...
async fn main() {
...
    tokio::spawn(sample_functions::stream_amazing_number());
}
```

Finally, receive the signals in Dart with `StreamBuilder` and rebuild the widget accordingly.

```dart title="lib/main.dart"
...
import 'package:example_app/messages/tutorial_resource.pb.dart';
...
children: [
  StreamBuilder(
    stream: MyAmazingNumber.rustSignalStream, // GENERATED
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text("Nothing received yet");
      }
      final myAmazingNumber = rustSignal.message;
      final currentNumber = myAmazingNumber.currentNumber;
      return Text(currentNumber.toString());
    },
  ),
...
```

## 🤝 Back and Forth

You can easily show the updated content on the screen by combining those two ways of message passing.

```proto title="messages/tutorial_resource.proto"
syntax = "proto3";
package tutorial_resource;
...
// [RINF:DART-SIGNAL]
message MyTreasureInput {}

// [RINF:RUST-SIGNAL]
message MyTreasureOutput { int32 current_value = 1; }
```

```bash title="CLI"
rinf message
```

```dart title="lib/main.dart"
...
import 'package:example_app/messages/tutorial_resource.pb.dart';
...
children: [
  StreamBuilder(
    stream: MyTreasureOutput.rustSignalStream, // GENERATED
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text('No value yet');
      }
      final myTreasureOutput = rustSignal.message;
      final currentNumber = myTreasureOutput.currentValue;
      return Text('Output value is $currentNumber');
    },
  ),
  ElevatedButton(
    onPressed: () {
      MyTreasureInput().sendSignalToRust(null); // GENERATED
    },
    child: Text('Send the input'),
  ),
...
```

```rust title="native/hub/src/sample_functions.rs"
...
use crate::messages;
...
pub async fn tell_treasure() {
    use messages::tutorial_resource::*;

    let mut current_value: i32 = 1;
    let mut receiver = MyTreasureInput::get_dart_signal_receiver(); // GENERATED
    while let Some(_) = receiver.recv().await {
        MyTreasureOutput { current_value }.send_signal_to_dart(None); // GENERATED
        current_value += 1;
    }
}
```

```rust title="native/hub/src/lib.rs"
...
mod sample_functions;
...
async fn main() {
...
    tokio::spawn(sample_functions::tell_treasure());
}
```

## ↩️ Awaiting a Response

If you want to store some state in Dart, you can implement something like a request-response pattern by providing a unique ID.

Please note that you don't need this technique if you're storing state in Rust.

```proto title="messages/tutorial_resource.proto"
syntax = "proto3";
package tutorial_resource;
...
// [RINF:DART-SIGNAL]
message MyUniqueInput {
  string uid = 1;
  int32 before_number = 2;
}

// [RINF:RUST-SIGNAL]
message MyUniqueOutput {
  string uid = 1;
  int32 before_number = 2;
}
```

```bash title="CLI"
rinf message
```

```dart title="lib/main.dart"
...
import 'package:uuid/uuid.dart';
import 'package:example_app/messages/tutorial_resource.pb.dart';
...
children: [
  ElevatedButton(
    onPressed: () async {
      final uid = Uuid().v4();
      MyUniqueInput(
        uid: uid,
        beforeNumber: 3,
      ).sendSignalToRust(null); // GENERATED
      final stream = MyUniqueOutput.rustSignalStream; // GENERATED
      final rustSignal = await stream.firstWhere((rustSignal) {
        return rustSignal.message.uid == uid;
      });
      print(rustSignal.message.afterNumber);
    },
    child: Text('Get the response'),
  ),
...
```

```rust title="native/hub/src/sample_functions.rs"
...
use crate::messages;
...
pub async fn respond_with_id() {
    use messages::tutorial_resource::*;

    let mut receiver = MyUniqueInput::get_dart_signal_receiver(); // GENERATED
    while let Some(dart_signal) = receiver.recv().await {
        let message = dart_signal.message;
        MyUniqueOutput {
            uid: message.uid,
            after_number: message.before_number + 3,
        }
        .send_signal_to_dart(None); // GENERATED
    }
}
```

```rust title="native/hub/src/lib.rs"
...
mod sample_functions;
...
async fn main() {
...
    tokio::spawn(sample_functions::respond_with_id());
}
```
