import 'dart:io';
import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';
import 'package:rinf/src/platform_utils.dart';

String? libPathOverride;

void overrideLibPath(String path) {
  libPathOverride = path;
}

RustLibrary loadRustLibrary() {
  // Use provided dynamic library path if possible.
  // Otherewise, use the default path.
  final override = libPathOverride;
  String libPath;
  if (override != null) {
    // Use the override path if provided.
    libPath = override;
  } else {
    // Load library files from the app bundle.
    if (Platform.isLinux) {
      libPath = 'libhub.so';
    } else if (Platform.isAndroid) {
      libPath = 'libhub.so';
    } else if (PlatformUtils.isOhos) {
      libPath = 'libhub.so';
    } else if (Platform.isWindows) {
      libPath = 'hub.dll';
    } else if (Platform.isIOS) {
      libPath = 'rinf.framework/rinf';
    } else if (Platform.isMacOS) {
      libPath = 'rinf.framework/rinf';
    } else {
      throw UnsupportedError('This operating system is not supported');
    }
  }

  // Load the dynamic library.
  final lib = DynamicLibrary.open(libPath);

  // Create the FFI wrapper instance.
  if (useLocalSpaceSymbols()) {
    return RustLibraryLocal(lib);
  } else {
    return RustLibraryGlobal(lib);
  }
}

/// Whether to search for symbols in the local space of dynamic library.
/// This is needed because of the different usage of
/// `RTLD_LOCAL` and `RTLD_GLOBAL` between platforms.
/// `RTLD_GLOBAL` is preferred as it has less overhead
/// because extra `malloc` can be avoided in Flutter.
bool useLocalSpaceSymbols() {
  // On Android, native library symbols are loaded in local space
  // because of Flutter's `RTLD_LOCAL` behavior.
  // Therefore we cannot use the efficient `RustLibraryGlobal`.
  // - https://github.com/dart-lang/native/issues/923
  if (Platform.isAndroid || PlatformUtils.isOhos) {
    return true;
  }

  // On Linux, `RTLD_LOCAL` behavior is required in tests
  // due to symbol resolution behavior observed across all distributions.
  // With `RTLD_GLOBAL`, symbols cannot be found.
  final isTest = Platform.environment.containsKey('FLUTTER_TEST');
  if (Platform.isLinux && isTest) {
    return true;
  }

  // Native library symbols are loaded in global space
  // thanks to Flutter's `RTLD_GLOBAL` behavior.
  return false;
}

/// The central interface for calling native function.
final rustLibrary = loadRustLibrary();

// Common type aliases.
// These exist for better readability of the code.

typedef PostCObjectInner = Int8 Function(Int64, Pointer<Dart_CObject>);
typedef PostCObjectPtr = Pointer<NativeFunction<PostCObjectInner>>;
typedef PrepareIsolateExtern = Void Function(PostCObjectPtr, Int64);
typedef PrepareIsolateWrapped = void Function(PostCObjectPtr, int);
typedef SendDartSignalExtern = Void Function(
    Pointer<Uint8>, UintPtr, Pointer<Uint8>, UintPtr);
typedef SendDartSignalWrapped = void Function(
    Pointer<Uint8>, int, Pointer<Uint8>, int);

/// Abstract class for unifying the interface
/// for calling native functions.
abstract class RustLibrary {
  void startRustLogic();
  void stopRustLogic();
  void prepareIsolate(PostCObjectPtr storePostObject, int port);
  void sendDartSignal(
    String endpointSymbol,
    Uint8List messageBytes,
    Uint8List binary,
  );
}

/// Class for global native library symbols loaded with `RTLD_GLOBAL`.
/// This is the efficient and ideal way to call native code.
/// `@Native` decorator with `isLeaf` parameter
/// that enables the `Uint8List.address` syntax
/// can only be used on globally loaded native symbols.
/// - https://github.com/dart-lang/sdk/issues/44589
/// - https://github.com/dart-lang/sdk/issues/44856
class RustLibraryGlobal extends RustLibrary {
  // Direct access to global function symbols loaded in the process.
  // These are available only if the native library is
  // loaded into global space with `RTLD_GLOBAL` configuration.

  final DynamicLibrary lib;
  final Map<String, SendDartSignalWrapped> sendDartSignalExterns = {};

  RustLibraryGlobal(this.lib);

  @Native<Void Function()>(isLeaf: true, symbol: 'rinf_start_rust_logic_extern')
  external static void startRustLogicExtern();

  @Native<Void Function()>(isLeaf: true, symbol: 'rinf_stop_rust_logic_extern')
  external static void stopRustLogicExtern();

  @Native<PrepareIsolateExtern>(
    isLeaf: true,
    symbol: 'rinf_prepare_isolate_extern',
  )
  external static void prepareIsolateExtern(
    PostCObjectPtr storePostObject,
    int port,
  );

  @override
  void startRustLogic() {
    startRustLogicExtern();
  }

  @override
  void stopRustLogic() {
    stopRustLogicExtern();
  }

  @override
  void prepareIsolate(PostCObjectPtr storePostObject, int port) {
    prepareIsolateExtern(storePostObject, port);
  }

  @override
  void sendDartSignal(
    String endpointSymbol,
    Uint8List messageBytes,
    Uint8List binary,
  ) {
    // Using the `@Native` annotation and avoiding `malloc`
    // with generated messages is not possible
    // because we cannot pass symbols dynamically.
    // Therefore, we search for the symbols in the dynamic library.
    // Also, Dart's native assets feature is not fully reliable yet.

    final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
    messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

    final Pointer<Uint8> binaryMemory = malloc.allocate(binary.length);
    binaryMemory.asTypedList(binary.length).setAll(0, binary);

    // Cache the dynamic library functions
    // to reduce symbol lookup overhead.
    var sendDartSignalExtern = sendDartSignalExterns[endpointSymbol];
    if (sendDartSignalExtern == null) {
      sendDartSignalExtern =
          lib.lookupFunction<SendDartSignalExtern, SendDartSignalWrapped>(
        endpointSymbol,
      );
      sendDartSignalExterns[endpointSymbol] = sendDartSignalExtern;
    }

    sendDartSignalExtern(
      messageMemory,
      messageBytes.length,
      binaryMemory,
      binary.length,
    );

    malloc.free(messageMemory);
    malloc.free(binaryMemory);
  }
}

/// Class for local native library symbols loaded with `RTLD_LOCAL`.
/// This is relatively inefficient because `malloc.allocate` is required.
/// It involves extra memory copy before sending the data to Rust.
class RustLibraryLocal extends RustLibrary {
  final DynamicLibrary lib;

  late void Function() startRustLogicExtern;
  late void Function() stopRustLogicExtern;
  late PrepareIsolateWrapped prepareIsolateExtern;
  final Map<String, SendDartSignalWrapped> sendDartSignalExterns = {};

  RustLibraryLocal(this.lib) {
    startRustLogicExtern = lib.lookupFunction<Void Function(), void Function()>(
      'rinf_start_rust_logic_extern',
    );
    stopRustLogicExtern = lib.lookupFunction<Void Function(), void Function()>(
      'rinf_stop_rust_logic_extern',
    );
    prepareIsolateExtern =
        lib.lookupFunction<PrepareIsolateExtern, PrepareIsolateWrapped>(
      'rinf_prepare_isolate_extern',
    );
  }

  @override
  void startRustLogic() {
    startRustLogicExtern();
  }

  @override
  void stopRustLogic() {
    stopRustLogicExtern();
  }

  @override
  void prepareIsolate(PostCObjectPtr storePostObject, int port) {
    prepareIsolateExtern(storePostObject, port);
  }

  @override
  void sendDartSignal(
    String endpointSymbol,
    Uint8List messageBytes,
    Uint8List binary,
  ) {
    final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
    messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

    final Pointer<Uint8> binaryMemory = malloc.allocate(binary.length);
    binaryMemory.asTypedList(binary.length).setAll(0, binary);

    // Cache the dynamic library functions
    // to reduce symbol lookup overhead.
    var sendDartSignalExtern = sendDartSignalExterns[endpointSymbol];
    if (sendDartSignalExtern == null) {
      sendDartSignalExtern =
          lib.lookupFunction<SendDartSignalExtern, SendDartSignalWrapped>(
        endpointSymbol,
      );
      sendDartSignalExterns[endpointSymbol] = sendDartSignalExtern;
    }

    sendDartSignalExtern(
      messageMemory,
      messageBytes.length,
      binaryMemory,
      binary.length,
    );

    malloc.free(messageMemory);
    malloc.free(binaryMemory);
  }
}
