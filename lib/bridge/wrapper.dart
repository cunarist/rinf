import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'dart:async';

var viewmodelUpdateBroadcaster = StreamController<String>.broadcast();

Future organizeRustRelatedThings() async {
  var endpointsOnRustThread = api.prepareChannels();
  endpointsOnRustThread.move = true;
  await api.layEndpointsOnRustThread(rustOpaque: endpointsOnRustThread);
  var viewmodelUpdateStream = api.prepareViewmodelUpdateStream();
  viewmodelUpdateStream.listen((event) {
    viewmodelUpdateBroadcaster.add(event);
  });
  await Future.delayed(const Duration(milliseconds: 100));
  api.startRustLogic();
}

Serialized? readViewmodel(String itemAddress) {
  Serialized? serialized = api.readViewmodel(
    itemAddress: itemAddress,
  );
  return serialized;
}

void sendUserAction(String taskAddress, Serialized serialized) {
  api.sendUserAction(
    taskAddress: taskAddress,
    serialized: serialized,
  );
}
