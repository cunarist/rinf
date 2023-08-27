import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';
import 'package:rust_in_flutter_example/messages/entry.pbserver.dart';

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
                return rustSignal.address == 'sample-category/mandelbrot';
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
    final requestMessage = CounterGetRequest(
      letter: "Hello from Dart!",
      beforeNumber: _count,
      dummyOne: 1,
      dummyTwo: 2,
      dummyThree: [3, 4, 5],
    );

    final rustRequest = RustRequest(
      address: 'basic-category/counter-number',
      operation: RustOperation.Read,
      // Convert Dart message object into raw bytes.
      bytes: requestMessage.writeToBuffer(),
    );
    // Use `requestToRust` from `rust_in_flutter.dart`
    // to send the request to Rust and get the response.
    final rustResponse = await requestToRust(rustRequest);

    if (!rustResponse.successful) {
      return;
    } else {
      // Convert raw bytes into Dart message objects.
      final responseMessage = CounterGetResponse.fromBuffer(rustResponse.bytes);
      _count = responseMessage.afterNumber;
      notifyListeners();
    }
  }
}
