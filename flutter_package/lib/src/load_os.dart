import 'dart:io' as io;
import 'dart:ffi';

String? dynamicLibPath;
final rustLibrary = loadRustLibrary();

void setDynamicLibPath(String path) {
  dynamicLibPath = path;
}

DynamicLibrary loadRustLibrary() {
  // Use provided dynamic library path if possible.
  final path = dynamicLibPath;
  if (path != null) {
    return DynamicLibrary.open(path);
  }

  // Otherewise, use the default path.
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
