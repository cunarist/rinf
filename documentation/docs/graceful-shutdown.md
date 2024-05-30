# Graceful Shutdown

When the Flutter app is closed, the entire `tokio` runtime on the Rust side will be terminated automatically. However, you might need to run some finalization code in Rust before the app closes. This might involve saving files or disposing of resources. To achieve this, you can call `finalizeRust()` in Dart to terminate all Rust tasks before closing the Flutter app.

```dart title="lib/main.dart"
import 'dart:ui';
import 'package:flutter/material.dart';
import './messages/generated.dart';
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
      await finalizeRust();
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
