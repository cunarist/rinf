# Graceful Shutdown

When the Flutter app is closed, the entire async runtime on the Rust side doesn't get dropped by default.

In some cases, you might need to drop all Rust resources properly before closing the app. This could include instances of structs that implement the `Drop` trait, which have roles like saving files or disposing of resources.

To achieve this, you can utilize Flutter's `AppLifecycleListener` to call the `finalizeRust` function before closing the Flutter app.

```{code-block} dart
:caption: lib/main.dart
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';

class MyApp extends StatefulWidget {
  const MyApp({super.key});
  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late final AppLifecycleListener _listener;

  @override
  void initState() {
    super.initState();
    _listener = AppLifecycleListener(
      onExitRequested: () async {
        finalizeRust(); // Shut down the async Rust runtime.
        return AppExitResponse.exit;
      },
    );
  }

  @override
  void dispose() {
    _listener.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // Return a widget.
  }
}
```

It's worth noting that `AppLifecycleListener` cannot always be relied upon for app closings. Below is a text snippet quoted from the official [Flutter docs](https://api.flutter.dev/flutter/widgets/State/dispose.html):

> There is no way to predict when application shutdown will happen. For example, a user's battery could catch fire, or the user could drop the device into a swimming pool, or the operating system could unilaterally terminate the application process due to memory pressure. Applications are responsible for ensuring they behave well even in the face of rapid, unscheduled termination.
