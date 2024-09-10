import 'dart:io' as io;
import 'dart:ffi';

String? dynamicLibPath;

void setDynamicLibPath(String path) {
  dynamicLibPath = path;
}

void loadRustLibrary() {
  // Use provided dynamic library path if possible.
  final path = dynamicLibPath;
  if (path != null) {
    DynamicLibrary.open(path);
  }

  // Otherewise, use the default path.
  if (io.Platform.isLinux) {
    DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isAndroid) {
    // DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isWindows) {
    DynamicLibrary.open('hub.dll');
  } else if (io.Platform.isIOS) {
    DynamicLibrary.open('rinf.framework/rinf');
  } else if (io.Platform.isMacOS) {
    DynamicLibrary.open('rinf.framework/rinf');
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }
}
