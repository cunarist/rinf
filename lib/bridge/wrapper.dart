import 'dart:typed_data';
import 'dart:convert';
import 'ffi.dart';
import 'dart:async';

var viewmodelUpdateStream = api.startAndGetViewmodelUpdateStream();
var viewmodelUpdateBroadcaster = StreamController<String>.broadcast();

dynamic readViewmodelAsJson(String itemAddress) {
  Uint8List? bytes = api.readViewmodel(
    itemAddress: itemAddress,
    takeOwnership: false,
  );
  dynamic jsonValue;
  if (bytes != null) {
    String jsonString = utf8.decode(bytes);
    jsonValue = json.decode(jsonString);
  } else {
    jsonValue = null;
  }
  return jsonValue;
}

Uint8List? readViewmodelAsBytes(String itemAddress,
    [bool takeOwnership = false]) {
  Uint8List? bytes = api.readViewmodel(
    itemAddress: itemAddress,
    takeOwnership: takeOwnership,
  );
  return bytes;
}

void sendUserAction(String taskAddress, dynamic jsonValue) {
  String jsonString = json.encode(jsonValue);
  api.sendUserAction(
    taskAddress: taskAddress,
    jsonString: jsonString,
  );
}
