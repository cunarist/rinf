# Detailed Techniques

## Signal Members

We've covered how to pass signals[^1] between Dart and Rust in the previous tutorial section. Now Let's delve into the meaning of each field of a signal.

- **Field `message`:** It represents a message of a type defined by Protobuf. This field is always filled.

- **Field `binary`:** This is a field designed to handle large binary data, potentially up to a few gigabytes. You can send any kind of binary data you wish, such as a high-resolution image or file data. This field carries empty `Uint8List` or `Vec<u8>` if the message is not marked as binary signal.

It's important to note that creating a Protobuf `message` larger than a few megabytes is not recommended. For large data, split them into multiple signals, or use the `binary` field instead.[^2]

[^1]: Rinf relies solely on native FFI for communication, avoiding the use of web protocols or hidden threads. The goal is to minimize performance overhead as much as possible.

[^2]: Sending a serialized message or binary data is a zero-copy operation from Rust to Dart, while it involves a copy operation from Dart to Rust in memory. Keep in mind that Protobuf's serialization and deserialization does involve memory copy.

## Generation Path

When you generate message code using the `rinf message` command, the resulting Dart and Rust modules' names and subpaths will precisely correspond to those of the `.proto` files.

- `./messages` : The `.proto` files under here and its subdirectories will be used.
- `./lib/messages` : The generated Dart code will be placed here.
- `./native/hub/src/messages` : The generated Rust code will be placed here.

## Continuous Watching

If you add the optional argument `-w` or `--watch` to the `rinf message` command, the message code will be automatically generated when `.proto` files are modified. If you add this argument, the command will not exit on its own.

```{code-block} shell
:caption: CLI
rinf message --watch
```

## Comments

It is possible to add comments like this.[^3]

```{code-block} proto
:caption: Protobuf
// This is a video data sample of...
// contains...
// responsible for...
message SomeData { bool my_field = 1; }
```

[^3]: If a message doesn't need a channel, it is entirely fine not to mark it with a special comment. In such instances, the message will still be generated without the ability to send signals. Generally, these messages are intended to be nested inside other messages.

This applies same to marked Protobuf messages.

```{code-block} proto
:caption: Protobuf
// [DART-SIGNAL]
// This is an audio data sample of...
// contains...
// responsible for...
message OtherData { bool my_field = 1; }
```
