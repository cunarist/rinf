# Unit Testing

## Pure Rust

Writing tests that only involve the business logic of Rust is relatively simple:

```{code-block} rust
:caption: Rust
#[tokio::test]
async fn my_async_test() {
  let result = async_function().await;
  assert_eq!(result, 42);
}

async fn async_function() -> i32 {
  42
}
```

```{code-block} shell
:caption: CLI
cargo test
```

## Dart and Rust

Writing tests that pass signals between Dart and Rust requires a few extra lines of code. First, you need to build the `hub` crate so that it is located in the `target` directory, and then load the dynamic library.

```{code-block} dart
:caption: test/custom_test.dart
import 'dart:io';
import 'package:test/test.dart';
import 'package:rinf/rinf.dart';
import 'package:my_app/src/bindings/bindings.dart';

void main() async {
  // Build the dynamic library and load it.
  await Process.run('cargo', ['build'], runInShell: true);
  await initializeRust(assignRustSignal, compiledLibPath: getLibPath());

  // Your test logic goes here.
  expect(42, 42);
  // ...
}


/// Gets the expected path of the dynamic library file.
/// The path should reflect the project folder structure.
String getLibPath() {
  if (Platform.isMacOS) {
    return 'target/debug/libhub.dylib';
  } else if (Platform.isLinux) {
    return 'target/debug/libhub.so';
  } else if (Platform.isWindows) {
    return 'target/debug/hub.dll';
  } else {
    throw UnsupportedError('This operating system is not for tests');
  }
}
```

```{code-block} shell
:caption: CLI
flutter test
```
