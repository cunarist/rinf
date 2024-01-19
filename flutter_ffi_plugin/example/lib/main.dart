import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import 'package:example_app/messages/generated.dart';
import 'package:example_app/messages/counter_number.pb.dart';
import 'package:example_app/messages/fractal_art.pb.dart';
import 'package:example_app/messages/tutorial_resource.pb.dart';

void main() async {
  // Wait for Rust initialization to be completed first.
  await Rinf.initialize(handleRustSignal);
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
      title: 'Rinf Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blueGrey,
          brightness: MediaQuery.platformBrightnessOf(context),
        ),
        useMaterial3: true,
      ),
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            StreamBuilder(
              stream: MyTreasureOutput.rustSignalStream, // GENERATED
              builder: (context, snapshot) {
                final rustSignal = snapshot.data;
                if (rustSignal == null) {
                  return Text('No value yet');
                }
                final currentNumber = rustSignal.message.currentValue;
                return Text('Output value is $currentNumber');
              },
            ),
            ElevatedButton(
              onPressed: () {
                MyTreasureInput().sendSignalToRust(null); // GENERATED
              },
              child: Text('Send the input'),
            ),
            StreamBuilder(
              stream: MyAmazingNumber.rustSignalStream, // GENERATED
              builder: (context, snapshot) {
                final rustSignal = snapshot.data;
                if (rustSignal == null) {
                  return Text("Nothing received yet");
                }
                final message = rustSignal.message;
                final currentNumber = message.currentNumber;
                return Text(currentNumber.toString());
              },
            ),
            ElevatedButton(
              onPressed: () async {
                MyPreciousData(
                  inputNumbers: [3, 4, 5],
                  inputString: 'Zero-cost abstraction',
                ).sendSignalToRust(null); // GENERATED
              },
              child: Text("Send a Signal from Dart to Rust"),
            ),
            // `StreamBuilder` listens to a stream
            // and rebuilds the widget accordingly.
            StreamBuilder(
                stream: SampleFractal.rustSignalStream,
                builder: (context, snapshot) {
                  final rustSignal = snapshot.data;
                  if (rustSignal == null) {
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
            StreamBuilder(
              // This stream is generated from a marked Protobuf message.
              stream: SampleNumberOutput.rustSignalStream,
              builder: (context, snapshot) {
                final rustSignal = snapshot.data;
                // If the app has just started and widget is built
                // without receiving a Rust signal,
                // the snapshot data will be null.
                // It's when the widget is being built for the first time.
                if (rustSignal == null) {
                  // Return the initial widget if the snapshot data is null.
                  return Text('Initial value 0');
                }
                final currentNumber = rustSignal.message.currentNumber;
                return Text('Current value is $currentNumber');
              },
            ),
          ],
        ),
      ),
      // This is a button that calls the generated function.
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          // The method is generated from a marked Protobuf message.
          SampleNumberInput(
            letter: "HELLO FROM DART!",
            dummyOne: 25,
            dummyTwo: SampleSchema(
              sampleFieldOne: true,
              sampleFieldTwo: false,
            ),
            dummyThree: [4, 5, 6],
          ).sendSignalToRust(null);
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
