import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'value.dart';
import 'bridge/wrapper.dart';
import 'dart:io';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'dart:typed_data';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    context.setLocale(const Locale('en', 'US'));

    // Return the actual app structure
    return MaterialApp(
      onGenerateTitle: (context) {
        if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
          appWindow.title = 'appTitle'.tr(); // For desktop
        }
        return 'appTitle'.tr(); // For mobile and web
      },
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          primary: primaryColor,
          secondary: secondaryColor,
        ),
      ),
      darkTheme: ThemeData(
        colorScheme: const ColorScheme.dark(
          primary: primaryColor,
          secondary: secondaryColor,
        ),
      ),
      themeMode: ThemeMode.system,
      home: const HomePage(),
      localizationsDelegates: context.localizationDelegates,
      supportedLocales: context.supportedLocales,
      locale: context.locale,
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            StreamBuilder<String>(
              stream: viewmodelUpdateBroadcaster.stream.where((itemAddress) {
                return itemAddress == 'someItemCategory.mandelbrot';
              }),
              builder: (context, snapshot) {
                if (snapshot.hasData) {
                  Uint8List? imageData = readViewmodelAsBytes(
                    'someItemCategory.mandelbrot',
                    true,
                  );
                  if (imageData != null) {
                    return Container(
                      margin: const EdgeInsets.all(20),
                      width: 256,
                      height: 256,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8.0),
                      ),
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(8),
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
                  } else {
                    return Container(
                      margin: const EdgeInsets.all(20),
                      width: 256,
                      height: 256,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8.0),
                        color: Colors.black,
                      ),
                    );
                  }
                } else {
                  return Container(
                    margin: const EdgeInsets.all(20),
                    width: 256,
                    height: 256,
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8.0),
                      color: Colors.black,
                    ),
                  );
                }
              },
            ),
            StreamBuilder<String>(
              stream: viewmodelUpdateBroadcaster.stream.where((itemAddress) {
                return itemAddress == 'someItemCategory.count';
              }),
              builder: (context, snapshot) {
                if (snapshot.hasData) {
                  Map? jsonValue = readViewmodelAsJson(
                    'someItemCategory.count',
                  );
                  String numberText = jsonValue?['value'].toString() ?? '??';
                  return Text('counter.informationText'.tr(namedArgs: {
                    'theValue': numberText,
                  }));
                } else {
                  return Text('counter.blankText'.tr());
                }
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Map jsonValue = {'dummy': null};
          sendUserAction(
            'someTaskCategory.calculateSomething',
            jsonValue,
          );
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
