import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import './messages/counter_number.pb.dart';
import './messages/fractal.pb.dart';
import './messages/handle.dart';

void main() async {
  // Wait for Rust initialization to be completed first.
  await Rinf.initialize(handleSignal);
  runApp(const MyApp());
}

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
      title: 'Rinf Example',
      theme: ThemeData(
        useMaterial3: true,
        brightness: MediaQuery.platformBrightnessOf(context),
      ),
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;

  void _incrementCounter() async {
    numberInputSend(
      NumberInput(
          letter: "HELLO FROM DART!",
          beforeNumber: _counter,
          dummyOne: 25,
          dummyTwo: SampleSchema(
            sampleFieldOne: true,
            sampleFieldTwo: false,
          ),
          dummyThree: [4, 5, 6]),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            // `StreamBuilder` listens to a stream
            // and rebuilds the widget accordingly.
            StreamBuilder<RustSignal<FractalScale>>(
                // Receive signals from Rust
                // with `rustBroadcaster` from `rinf.dart`,
                // For better performance, filter signals
                // by checking the `resource` field with the `where` method.
                // This approach allows the builder to rebuild its widget
                // only when there are signals
                // related to a specific Rust resource it is interested in.
                stream: fractalScaleStream,
                builder: (context, snapshot) {
                  // If the app has just started and widget is built
                  // without receiving a Rust signal,
                  // the snapshot's data will be null.
                  final rustSignal = snapshot.data;
                  if (rustSignal == null) {
                    // Return a black container if the received data is null.
                    return Container(
                      margin: const EdgeInsets.all(20),
                      width: 256,
                      height: 256,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(24.0),
                        color: Colors.black,
                      ),
                    );
                  }
                  // Return an image container if some data is received.
                  final imageData = rustSignal.blob!;
                  return Container(
                    margin: const EdgeInsets.all(20),
                    width: 256,
                    height: 256,
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(24.0),
                      child: FittedBox(
                        fit: BoxFit.contain,
                        child: Image.memory(
                          imageData,
                          width: 256,
                          height: 256,
                          gaplessPlayback: true,
                        ),
                      ),
                    ),
                  );
                }),
            StreamBuilder<RustSignal<NumberOutput>>(
              stream: numberOutputStream,
              builder: (context, snapshot) {
                final rustSignal = snapshot.data;
                if (rustSignal == null) {
                  return Text('Initial value 0');
                }
                _counter = rustSignal.message.afterNumber;
                return Text('Current value is $_counter');
              },
            ),
          ],
        ),
      ),
      // This is a button that calls the increment method.
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
