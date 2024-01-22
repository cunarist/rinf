# Writing Code

## 🏷️ Signal Details

### Meanings of Each Field

We've covered how to pass signals between Dart and Rust in the previous tutorial section. Now Let's delve into the meaning of each field of a signal.

- **Field `message`:** It represents a message of a type defined by Protobuf. This field is mandatory.

- **Field `blob`:** This is a field designed to handle large binary data, potentially up to a few gigabytes. You can send any kind of binary data you wish, such as a high-resolution image or file data. This field is optional and can be set to `null` or `None`.

It's important to note that creating a Protobuf `message` larger than a few megabytes is not recommended. For large data, split them into multiple signals, or use `blob` instead.

### Efficiency

Rinf relies solely on native FFI for communication, avoiding the use of web protocols or hidden threads. The goal is to minimize performance overhead as much as possible.

Sending a serialized message or blob data is a zero-copy operation from Rust to Dart, while it involves a copy operation from Dart to Rust in memory. Keep in mind that Protobuf's serialization and deserialization does involve memory copy.

## 📦 Message Details

### Generated Path

When you generate message code using the `rinf message` command, the resulting Dart and Rust modules' names and subpaths will precisely correspond to those of the `.proto` files.

- `./messages` : The `.proto` files under here and its subdirectories will be used.
- `./lib/messages` : The generated Dart code will be placed here.
- `./native/hub/src/messages` : The generated Rust code will be placed here.

### Continuous Watching

If you add the optional argument `-w` or `--watch` to the `rinf message` command, the message code will be automatically generated when `.proto` files are modified. If you add this argument, the command will not exit on its own.

```bash title="CLI"
rinf message --watch
```

### Normal Messages

If a message doesn't need a channel, then it is totally fine not to mark it with a special comment at all. In that case, the message will still be generated without the ability to send signals. In general, they would be nested inside other messages.

### Comments

It is possible to add comments like this.

```proto title="Protobuf"
// This is a video data sample of...
// contains...
// responsible for...
message SomeData { ... }
```

This applies same to marked Protobuf messages.

```proto title="Protobuf"
// [RINF:DART-SIGNAL]
// This is an audio data sample of...
// contains...
// responsible for...
message OtherData { ... }
```

## 🖨️ Printing for Debugging

You might be used to `println!` macro in Rust. However, using that macro isn't a very good idea in our apps made with Flutter and Rust because `println!` outputs cannot be seen on the web and mobile emulators.

When writing Rust code in the `hub` crate, you can simply print your debug message with the `debug_print!` macro provided by this framework like below. Once you use this macro, Flutter will take care of the rest.

```rust title="Rust"
use crate::debug_print;
debug_print!("My object is {my_object:?}");
```

`debug_print!` is also better than `println!` because it only works in debug mode, resulting in a smaller and cleaner release binary.

## 🌅 Closing the App Gracefully

When the Flutter app is closed, the whole `tokio` runtime on the Rust side will be terminated automatically. However, some error messages can appear in the console if the Rust side sends messages to the Dart side even after the Dart VM has stopped. To prevent this, you can call `Rinf.finalize()` in Dart to terminate all Rust tasks before closing the Flutter app.

```dart title="lib/main.dart"
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
      await Rinf.finalize();
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
      title: 'Some App',
      home: MyHomePage(),
    );
  }
}
...
```
