# Tutorial

To grasp the basic concepts, it's beneficial to follow a step-by-step tutorial.

Before we start, make sure that there's a `Column` somewhere in your widget tree. This will contain our tutorial widgets.

```{code-block} dart
:caption: lib/main.dart
Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [],
)
```

## From Dart to Rust

Let's say that you want to create a new button in Dart that sends an array of numbers and a string to Rust. We need a signal to notify Rust that a user event has occurred.

Write a new signal struct in the `hub` crate. Note that the message should have the attribute `DartSignal` above it.

```{code-block} rust
:caption: native/hub/src/signals/mod.rs
use rinf::DartSignal;
use serde::Deserialize;

#[derive(Deserialize, DartSignal)]
pub struct MyPreciousData {
  pub input_numbers: Vec<i32>,
  pub input_string: String,
}
```

Next, generate Dart code from annotated signal structs.

```{code-block} shell
:caption: CLI
rinf gen
```

Create a button widget in Dart that accepts the user input.

```{code-block} dart
:caption: lib/main.dart
import 'package:my_app/src/bindings/bindings.dart';

Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {
        MyPreciousData(
          inputNumbers: [3, 4, 5],
          inputString: 'Zero-cost abstraction',
        ).sendSignalToRust(); // GENERATED
      },
      child: Text('Send a Signal from Dart to Rust'),
    ),
  ]
)
```

Let's listen to this message in Rust. This simple function will add one to each element in the array and capitalize all letters in the string.

```{code-block} rust
:caption: native/hub/src/tutorial_functions.rs
use crate::signals::MyPreciousData;
use rinf::{DartSignal, debug_print};

pub async fn calculate_precious_data() {
  let receiver = MyPreciousData::get_dart_signal_receiver(); // GENERATED
  while let Some(signal_pack) = receiver.recv().await {
    let my_precious_data = signal_pack.message;

    let new_numbers: Vec<i32> = my_precious_data
      .input_numbers
      .into_iter()
      .map(|x| x + 1)
      .collect();
    let new_string = my_precious_data.input_string.to_uppercase();

    debug_print!("{:?}", new_numbers);
    debug_print!("{}", new_string);
  }
}
```

```{code-block} rust
:caption: native/hub/src/lib.rs
mod tutorial_functions;

use tokio::spawn;
use tutorial_functions::calculate_precious_data;

#[tokio::main]
async fn main() {
  spawn(calculate_precious_data());
  dart_shutdown().await;
}
```

Now run the app with `flutter run`. We can see the printed output in the command-line when clicking the button!

```{code-block} none
:caption: Output
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

## From Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart.

Define the signal struct. Note that the struct should have the attribute `RustSignal` above it.

```{code-block} rust
:caption: native/hub/src/signals/mod.rs
use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct MyAmazingNumber {
  pub current_number: i32,
}
```

Generate Dart signal classes.

```{code-block} shell
:caption: CLI
rinf gen
```

Define an async Rust function that runs forever, sending numbers to Dart every second.

```{code-block} rust
:caption: native/hub/src/tutorial_functions.rs
use crate::signals::MyAmazingNumber;
use rinf::RustSignal;
use std::time::Duration;
use tokio::time::interval;

pub async fn stream_amazing_number() {
  let mut current_number: i32 = 1;
  let mut time_interval = interval(Duration::from_secs(1));
  loop {
    time_interval.tick().await;
    MyAmazingNumber { current_number }.send_signal_to_dart(); // GENERATED
    current_number += 1;
  }
}
```

```{code-block} rust
:caption: native/hub/src/lib.rs
mod tutorial_functions;

use tokio::spawn;
use tutorial_functions::stream_amazing_number;

#[tokio::main]
async fn main() {
  // ...
  tokio::spawn(stream_amazing_number());
  dart_shutdown().await;
}
```

Finally, receive the signals in Dart with `StreamBuilder` and rebuild the widget accordingly.

```{code-block} dart
:caption: lib/main.dart
import 'package:my_app/src/bindings/bindings.dart';

Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    // ...
    StreamBuilder(
      stream: MyAmazingNumber.rustSignalStream, // GENERATED
      builder: (context, snapshot) {
        final signalPack = snapshot.data;
        if (signalPack == null) {
          return Text('Nothing received yet');
        }
        final myAmazingNumber = signalPack.message;
        final currentNumber = myAmazingNumber.currentNumber;
        return Text(currentNumber.toString());
      },
    ),
  ],
)
```

## Back and Forth

You can easily show the updated state on the screen by combining those two ways of message passing.

```{code-block} rust
:caption: native/hub/src/signals/mod.rs
use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct MyTreasureInput {}

#[derive(Serialize, RustSignal)]
pub struct MyTreasureOutput {
  pub current_value: i32,
}
```

```{code-block} shell
:caption: CLI
rinf gen
```

```{code-block} dart
:caption: lib/main.dart
import 'package:my_app/src/bindings/bindings.dart';

children: [
  // ...
  StreamBuilder(
    stream: MyTreasureOutput.rustSignalStream, // GENERATED
    builder: (context, snapshot) {
      final signalPack = snapshot.data;
      if (signalPack == null) {
        return Text('No value yet');
      }
      final myTreasureOutput = signalPack.message;
      final currentNumber = myTreasureOutput.currentValue;
      return Text('Output value is $currentNumber');
    },
  ),
  ElevatedButton(
    onPressed: () {
      MyTreasureInput().sendSignalToRust(); // GENERATED
    },
    child: Text('Send the input'),
  ),
]
```

```{code-block} rust
:caption: native/hub/src/tutorial_functions.rs
use crate::signals::{MyTreasureInput, MyTreasureOutput};
use rinf::{DartSignal, RustSignal};

pub async fn tell_treasure() {
  let mut current_value: i32 = 1;

  let receiver = MyTreasureInput::get_dart_signal_receiver(); // GENERATED
  while let Some(_) = receiver.recv().await {
    MyTreasureOutput { current_value }.send_signal_to_dart(); // GENERATED
    current_value += 1;
  }
}
```

```{code-block} rust
:caption: native/hub/src/lib.rs
mod tutorial_functions;

use tokio::spawn;
use tutorial_functions::tell_treasure;

#[tokio::main]
async fn main() {
  // ...
  spawn(tell_treasure());
  dart_shutdown().await;
}
```
