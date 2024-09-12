import 'dart:io' as io;
import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';

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
  } else if (io.Platform.isLinux) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isAndroid) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isWindows) {
    lib = DynamicLibrary.open('hub.dll');
  } else if (io.Platform.isIOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else if (io.Platform.isMacOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }

  if (io.Platform.isAndroid) {
    // On Android, native library symbols are loaded in local space
    // because of Flutter's `RTLD_LOCAL` behavior.
    // Therefore we cannot use the efficient `RustLibraryNew`.
    // - https://github.com/dart-lang/native/issues/923
    return RustLibraryOld(lib);
  } else {
    // Native library symbols are loaded in global space
    // thanks to Flutter's `RTLD_GLOBAL` behavior.
    return RustLibraryNew();
  }
}

// The central interface for calling native function.

final rustLibrary = loadRustLibrary();

// Common type aliases.
// This is for better readability of the code.

typedef PostCObjectInner = Int8 Function(Int64, Pointer<Dart_CObject>);
typedef PostCObjectFn = NativeFunction<PostCObjectInner>;

// Direct access to global function symbols loaded in the process.
// These are available only if the native library is
// loaded into global space with `RTLD_GLOBAL` configuration.

@Native<Void Function()>(
  isLeaf: true,
  symbol: 'start_rust_logic_extern',
)
external void startRustLogicExtern();

@Native<Void Function()>(
  isLeaf: true,
  symbol: 'stop_rust_logic_extern',
)
external void stopRustLogicExtern();

typedef SendDartSignalExtern = Void Function(
  Int32,
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);
@Native<SendDartSignalExtern>(
  isLeaf: true,
  symbol: 'send_dart_signal_extern',
)
external void sendDartSignalExtern(
  int messageId,
  Pointer<Uint8> messageBytesAddress,
  int messageBytesLength,
  Pointer<Uint8> binaryAddress,
  int binaryLength,
);

@Native<Void Function(Int64)>(
  isLeaf: true,
  symbol: 'prepare_isolate_extern',
)
external void prepareIsolateExtern(
  int port,
);

@Native<Void Function(Pointer<PostCObjectFn>)>(
  isLeaf: true,
  symbol: 'store_dart_post_cobject',
)
external void storeDartPostCObjectExtern(
  Pointer<PostCObjectFn> postCObject,
);

/// Abstract class for unifying the interface
/// for calling native functions.
abstract class RustLibrary {
  void startRustLogic();
  void stopRustLogic();
  void sendDartSignal(
    int messageId,
    Uint8List messageBytes,
    Uint8List binary,
  );
  void prepareIsolate(int port);
  void storeDartPostCObject(Pointer<PostCObjectFn> postCObject);
}

/// Class for global native library symbols loaded with `RTLD_GLOBAL`.
/// This is the efficient and ideal way to call native code.
/// `@Native` decorator with `isLeaf` parameter
/// that enables the `Uint8List.address` syntax
/// can only used on global native symbols.
/// - https://github.com/dart-lang/sdk/issues/44589
class RustLibraryNew extends RustLibrary {
  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void sendDartSignal(
    int messageId,
    Uint8List messageBytes,
    Uint8List binary,
  ) {
    sendDartSignalExtern(
      messageId,
      messageBytes.address,
      messageBytes.length,
      binary.address,
      binary.length,
    );
  }

  void prepareIsolate(int port) {
    prepareIsolateExtern(port);
  }

  void storeDartPostCObject(Pointer<PostCObjectFn> postCObject) {
    storeDartPostCObjectExtern(postCObject);
  }
}

/// Class for local native library symbols loaded with `RTLD_LOCAL`.
/// This is relatively inefficient because `malloc.allocate` is required.
class RustLibraryOld extends RustLibrary {
  final DynamicLibrary lib;
  RustLibraryOld(this.lib);

  void storeDartPostCObject(Pointer<PostCObjectFn> postCObject) {
    final rustFunction = lib.lookupFunction<
        Pointer Function(Pointer<PostCObjectFn>),
        Pointer Function(Pointer<PostCObjectFn>)>(
      'store_dart_post_cobject',
    );
    rustFunction(postCObject);
  }

  void startRustLogic() {
    final rustFunction = lib.lookupFunction<Void Function(), void Function()>(
      'start_rust_logic_extern',
    );
    rustFunction();
  }

  void stopRustLogic() {
    final rustFunction = lib.lookupFunction<Void Function(), void Function()>(
      'stop_rust_logic_extern',
    );
    rustFunction();
  }

  void sendDartSignal(int messageId, Uint8List messageBytes, Uint8List binary) {
    final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
    messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

    final Pointer<Uint8> binaryMemory = malloc.allocate(binary.length);
    binaryMemory.asTypedList(binary.length).setAll(0, binary);

    final rustFunction = lib.lookupFunction<
        Void Function(Int32, Pointer<Uint8>, UintPtr, Pointer<Uint8>, UintPtr),
        void Function(int, Pointer<Uint8>, int, Pointer<Uint8>, int)>(
      'send_dart_signal_extern',
    );

    rustFunction(
      messageId,
      messageMemory,
      messageBytes.length,
      binaryMemory,
      binary.length,
    );

    malloc.free(messageMemory);
    malloc.free(binaryMemory);
  }

  void prepareIsolate(int port) {
    final rustFunction =
        lib.lookupFunction<Void Function(Int64), void Function(int)>(
      'prepare_isolate_extern',
    );
    rustFunction(port);
  }
}
