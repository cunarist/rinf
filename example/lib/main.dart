import 'package:flutter/material.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';
import 'package:example_app/messages/counter_number.pb.dart' as counterNumber;
import 'package:example_app/messages/mandelbrot.pb.dart' as mandelbrot;

void main() async {
  // Wait for initialization to be completed first.
  await RustInFlutter.ensureInitialized();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Home(),
    );
  }
}

class Home extends StatelessWidget {
  final ValueNotifier<int> _countNotifier = ValueNotifier<int>(0);

  // This method interacts with Rust.
  void _incrementCount() async {
    final requestMessage = counterNumber.ReadRequest(
      letter: "Hello from Dart!",
      beforeNumber: _countNotifier.value,
      dummyOne: 1,
      dummyTwo: counterNumber.SampleSchema(
        sampleFieldOne: true,
        sampleFieldTwo: false,
      ),
      dummyThree: [3, 4, 5],
    );

    final rustRequest = RustRequest(
      resource: counterNumber.ID,
      operation: RustOperation.Read,
      // Convert Dart message object into raw bytes.
      message: requestMessage.writeToBuffer(),
    );

    // Use `requestToRust` from `rust_in_flutter.dart`
    // to send the request to Rust and get the response.
    final rustResponse = await requestToRust(rustRequest);

    if (rustResponse.successful) {
      // Convert raw bytes into Dart message objects.
      final responseMessage =
          counterNumber.ReadResponse.fromBuffer(rustResponse.message!);
      _countNotifier.value = responseMessage.afterNumber;
    }
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
            StreamBuilder<RustSignal>(
              // Receive signals from Rust
              // with `rustBroadcaster` from `rust_in_flutter.dart`,
              // For better performance, filter signals
              // by checking the `address` field with the `where` method.
              // This approach allows the builder to rebuild its widget
              // only when there are signals
              // with the specific address it is interested in.
              stream: rustBroadcaster.stream.where((rustSignal) {
                return rustSignal.resource == mandelbrot.ID;
              }),
              builder: (context, snapshot) {
                // If the app has just started and widget is built
                // without receiving a Rust signal,
                // the snapshot's data will be null.
                final received = snapshot.data;
                if (received == null) {
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
                } else {
                  // Return an image container if some data is received.
                  final imageData = received.blob!;
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
                          width: 64,
                          height: 64,
                          gaplessPlayback: true,
                        ),
                      ),
                    ),
                  );
                }
              },
            ),
            CurrentValueText(
              countNotifier: _countNotifier,
            ), // Display current value
          ],
        ),
      ),
      // This is a button that calls the increment method.
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCount,
        child: const Icon(Icons.add),
      ),
    );
  }
}

class CurrentValueText extends StatefulWidget {
  final ValueNotifier<int> countNotifier;
  const CurrentValueText({required this.countNotifier});
  @override
  _CurrentValueTextState createState() => _CurrentValueTextState();
}

class _CurrentValueTextState extends State<CurrentValueText> {
  late int _currentCount;

  @override
  void initState() {
    super.initState();
    _currentCount = widget.countNotifier.value;
    widget.countNotifier.addListener(_updateCurrentCount);
  }

  void _updateCurrentCount() {
    setState(() {
      _currentCount = widget.countNotifier.value;
    });
  }

  @override
  void dispose() {
    widget.countNotifier.removeListener(_updateCurrentCount);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_currentCount == 0) {
      return const Text("Not calculated yet");
    } else {
      return Text(
        "Current value is $_currentCount",
      );
    }
  }
}
