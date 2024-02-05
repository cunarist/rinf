import 'dart:io' as io;
import 'dart:ffi';

final rustLibrary = loadRustLibrary();

DynamicLibrary loadRustLibrary() {
  if (io.Platform.isLinux) {
    return DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isAndroid) {
    return DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isWindows) {
    return DynamicLibrary.open('hub.dll');
  } else if (io.Platform.isIOS) {
    return DynamicLibrary.open('rinf.framework/rinf');
  } else if (io.Platform.isMacOS) {
    return DynamicLibrary.open('rinf.framework/rinf');
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }
}
