import 'dart:io';
import 'dart:ffi';

String? dynamicLibPath;

void setDynamicLibPath(String path) {
  dynamicLibPath = path;
}

RustLibrary loadRustLibrary() {
  // Use provided dynamic library path if possible.
  // Otherewise, use the default path.
  final path = dynamicLibPath;
  DynamicLibrary lib;
  if (path != null) {
    lib = DynamicLibrary.open(path);
  } else if (Platform.isLinux) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (Platform.isAndroid) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (Platform.isWindows) {
    lib = DynamicLibrary.open('hub.dll');
  } else if (Platform.isIOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else if (Platform.isMacOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }

  // On Android, native library symbols are loaded in local space
  // because of Flutter's `RTLD_LOCAL` behavior.
  // Therefore we cannot use the efficient `RustLibraryGlobal`.
  // - https://github.com/dart-lang/native/issues/923
  if (Platform.isAndroid) {
    return RustLibraryLocal(lib: lib);
  }

  // On Linux, `RTLD_LOCAL` behavior is required in tests
  // due to symbol resolution behavior observed across all distributions.
  // With `RTLD_GLOBAL`, symbols cannot be found.
  final isTest = Platform.environment.containsKey('FLUTTER_TEST');
  if (Platform.isLinux && isTest) {
    return RustLibraryLocal(lib: lib);
  }

  // Native library symbols are loaded in global space
  // thanks to Flutter's `RTLD_GLOBAL` behavior.
  return RustLibraryGlobal();
}

/// The central interface for calling native function.
final rustLibrary = loadRustLibrary();

// Common type aliases.
// These exist for better readability of the code.

typedef PostCObjectInner = Int8 Function(Int64, Pointer<Dart_CObject>);
typedef PostCObjectPtr = Pointer<NativeFunction<PostCObjectInner>>;
typedef PrepareIsolateExtern = Void Function(PostCObjectPtr, Int64);
typedef PrepareIsolateWrap = void Function(PostCObjectPtr, int);

/// Abstract class for unifying the interface
/// for calling native functions.
abstract class RustLibrary {
  void startRustLogic();
  void stopRustLogic();
  void prepareIsolate(PostCObjectPtr storePostObject, int port);
}

/// Class for global native library symbols loaded with `RTLD_GLOBAL`.
/// This is the efficient and ideal way to call native code.
/// `@Native` decorator with `isLeaf` parameter
/// that enables the `Uint8List.address` syntax
/// can only be used on globally loaded native symbols.
/// - https://github.com/dart-lang/sdk/issues/44589
class RustLibraryGlobal extends RustLibrary {
  // Direct access to global function symbols loaded in the process.
  // These are available only if the native library is
  // loaded into global space with `RTLD_GLOBAL` configuration.

  @Native<Void Function()>(
    isLeaf: true,
    symbol: 'rinf_start_rust_logic_extern',
  )
  external static void startRustLogicExtern();

  @Native<Void Function()>(
    isLeaf: true,
    symbol: 'rinf_stop_rust_logic_extern',
  )
  external static void stopRustLogicExtern();

  @Native<Void Function(PostCObjectPtr, Int64)>(
    isLeaf: true,
    symbol: 'rinf_prepare_isolate_extern',
  )
  external static void prepareIsolateExtern(
    PostCObjectPtr storePostObject,
    int port,
  );

  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void prepareIsolate(PostCObjectPtr storePostObject, int port) {
    prepareIsolateExtern(storePostObject, port);
  }
}

/// Class for local native library symbols loaded with `RTLD_LOCAL`.
/// This is relatively inefficient because `malloc.allocate` is required.
/// It involves extra memory copy before sending the data to Rust.
class RustLibraryLocal extends RustLibrary {
  final DynamicLibrary lib;
  late void Function() startRustLogicExtern;
  late void Function() stopRustLogicExtern;
  late void Function(PostCObjectPtr, int) prepareIsolateExtern;

  RustLibraryLocal({required this.lib}) {
    this.startRustLogicExtern =
        lib.lookupFunction<Void Function(), void Function()>(
      'rinf_start_rust_logic_extern',
    );
    this.stopRustLogicExtern =
        lib.lookupFunction<Void Function(), void Function()>(
      'rinf_stop_rust_logic_extern',
    );
    this.prepareIsolateExtern =
        lib.lookupFunction<PrepareIsolateExtern, PrepareIsolateWrap>(
      'rinf_prepare_isolate_extern',
    );
  }

  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void prepareIsolate(PostCObjectPtr storePostObject, int port) {
    prepareIsolateExtern(storePostObject, port);
  }
}
