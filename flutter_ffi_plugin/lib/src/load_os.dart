import 'dart:io' as io;
import 'dart:ffi';

final DynamicLibrary rustLibrary = loadNativeLibrary();

DynamicLibrary loadNativeLibrary() {
  if (io.Platform.isLinux) {
    return DynamicLibrary.open('libhub.so'); // Dynamic library
  } else if (io.Platform.isAndroid) {
    return DynamicLibrary.open('libhub.so'); // Dynamic library
  } else if (io.Platform.isWindows) {
    return DynamicLibrary.open('hub.dll'); // Dynamic library
  } else if (io.Platform.isIOS) {
    return DynamicLibrary.executable(); // Static library
  } else if (io.Platform.isMacOS) {
    return DynamicLibrary.executable(); // Static library
  } else {
    throw UnsupportedError('The operating system is not supported.');
  }
}
