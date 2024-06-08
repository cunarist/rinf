# Graceful Shutdown

When the Flutter app is closed, the entire `tokio` async runtime on the Rust side will be terminated automatically.

When using Rinf, the lifetime of the `tokio` runtime follows that of the Dart runtime. This behavior is different from typical `tokio` executables where its async runtime lives throughout the `main()` function of Rust.

In some cases, you might need to run some finalization code in Rust before the app closes. This might involve saving files or disposing of resources. To achieve this, you can use Flutter's `AppLifecycleListener` to run something or to get user confirmation before closing the Flutter app.

```dart title="lib/main.dart"
import 'dart:ui';
import 'package:flutter/material.dart';
...
class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final _appLifecycleListener = AppLifecycleListener(
    onExitRequested: () async {
      // Do something here before the app is exited.
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
