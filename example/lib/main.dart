import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:msgpack_dart/msgpack_dart.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';

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
      // This example uses providers for very simple state management.
      home: ChangeNotifierProvider(
        create: (context) => HomeNotifier(),
        child: const Home(),
      ),
    );
  }
}

class Home extends StatelessWidget {
  const Home({super.key});

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
                return rustSignal.address == 'sampleCategory.mandelbrot';
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
                  final imageData = received.bytes;
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
            // Update the text according to the state.
            Consumer<HomeNotifier>(
              builder: (context, notifier, child) {
                final currentCount = notifier.count;
                if (currentCount == 0) {
                  return const Text("Not calculated yet");
                } else {
                  return Text(
                    "Current value is ${currentCount.toString()}",
                  );
                }
              },
            )
          ],
        ),
      ),
      // This is a button that calls the state update method.
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          context.read<HomeNotifier>().increment();
        },
        child: const Icon(Icons.add),
      ),
    );
  }
}

class HomeNotifier extends ChangeNotifier {
  int _count = 0;
  int get count => _count;

  // This state update method interacts with Rust.
  void increment() async {
    final rustRequest = RustRequest(
      address: 'basicCategory.counterNumber',
      operation: RustOperation.Read,
      // Use the `serialize` function
      // provided by `msgpack_dart.dart`
      // to convert Dart objects into raw bytes.
      bytes: serialize(
        // The message schema is declared at the Rust side.
        // You can customize the schemas as you want.
        {
          'letter': 'Hello from Dart!',
          'before_number': _count,
          'dummy_one': 1,
          'dummy_two': 2,
          'dummy_three': [3, 4, 5]
        },
      ),
    );
    // Use `requestToRust` from `rust_in_flutter.dart`
    // to send the request to Rust and get the response.
    final rustResponse = await requestToRust(rustRequest);
    if (!rustResponse.successful) {
      return;
    }
    // Use the `deserialize` function
    // provided by `msgpack_dart.dart`
    // to convert raw bytes into Dart objects.
    // You have to explicitly tell the deserialized type
    // with `as` keyword for proper type checking in Dart.
    final message = deserialize(rustResponse.bytes) as Map;
    final innerValue = message['after_number'] as int;
    _count = innerValue;
    notifyListeners();
  }
}
