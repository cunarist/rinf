import 'dart:typed_data';
import 'dart:convert';
import 'ffi.dart';
import 'dart:async';

var viewmodelUpdateStream = api.startAndGetViewmodelUpdateStream();
var viewmodelUpdateBroadcaster = StreamController<String>.broadcast();

DotAddress toDotAddress(String plain) {
  return DotAddress(layered: plain.split('.'));
}

dynamic readViewmodelAsJson(String dataAddress) {
  Uint8List? bytes = api.readViewmodel(
    dataAddress: toDotAddress(dataAddress),
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

Uint8List? readViewmodelAsBytes(String dataAddress,
    [bool takeOwnership = false]) {
  Uint8List? bytes = api.readViewmodel(
    dataAddress: toDotAddress(dataAddress),
    takeOwnership: takeOwnership,
  );
  return bytes;
}

void sendUserAction(String taskAddress, dynamic jsonValue) {
  String jsonString = json.encode(jsonValue);
  api.sendUserAction(
    taskAddress: toDotAddress(taskAddress),
    jsonString: jsonString,
  );
}
