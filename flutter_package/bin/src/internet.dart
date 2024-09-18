import 'dart:io';

var isInternetConnected = false;

Future<void> checkConnectivity() async {
  try {
    final result = await InternetAddress.lookup('pub.dev');
    if (result.isNotEmpty && result[0].rawAddress.isNotEmpty) {
      isInternetConnected = true;
    }
  } on SocketException catch (_) {
    isInternetConnected = false;
  }
}
