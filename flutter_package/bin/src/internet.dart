import 'package:internet_connection_checker/internet_connection_checker.dart';

var isInternetConnected = false;

Future<void> checkConnectivity() async {
  isInternetConnected = await InternetConnectionChecker().hasConnection;
}
