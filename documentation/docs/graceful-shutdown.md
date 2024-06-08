# Graceful Shutdown

When the Flutter app is closed, the entire `tokio` async runtime on the Rust side will be terminated automatically. Even if the app is force-closed, the `tokio` async runtime will be properly dropped.

When using Rinf, the lifetime of the `tokio` runtime follows that of the Dart runtime. This behavior is different from typical `tokio` executables where its async runtime lives throughout the async `main()` function of Rust.

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

It's worth noting that `AppLifecycleListener` or `dispose` cannot always be relied upon for app closings. Below is a text snippet quoted from the official [Flutter docs](https://api.flutter.dev/flutter/widgets/State/dispose.html):

> There is no way to predict when application shutdown will happen. For example, a user's battery could catch fire, or the user could drop the device into a swimming pool, or the operating system could unilaterally terminate the application process due to memory pressure. Applications are responsible for ensuring they behave well even in the face of rapid, unscheduled termination.
