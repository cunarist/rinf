import 'dart:async';

import 'io.dart' if (dart.library.html) 'web.dart'
    show DartPostCObject, NativePortType, WireSyncReturn;
export 'io.dart' if (dart.library.html) 'web.dart'
    show
        ExternalLibrary,
        WireSyncReturn,
        FrbOpaqueBase,
        DartApiDl,
        NativePortType,
        PlatformPointer,
        OpaqueTypeFinalizer;
import 'package:rinf/src/engine/isolate.dart' show SendPort;

/// This class, together with its subclasses, are only for internal usage.
/// Usually it should not be used by normal users.
abstract class FlutterRustBridgeWireBase {
  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(DartPostCObject ptr) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj) {
    throw UnimplementedError();
  }

  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn val) {
    throw UnimplementedError();
  }
}

extension NativeType on SendPort {
  NativePortType get nativePort => throw UnimplementedError();
}

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() => throw UnimplementedError();
}

/// Generates the dynamic Dart object from either an FFI struct or a JS value
List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) =>
    throw UnimplementedError();

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
bool? get crossOriginIsolated => throw UnimplementedError();

int castInt(Object? value) => value as int;

/// Only used on the Web.
Object castNativeBigInt(int value) => throw UnimplementedError();

abstract class FlutterRustBridgeWasmWireBase<T extends WasmModule>
    extends FlutterRustBridgeWireBase {
  Future<T> get init => throw UnimplementedError();
  FlutterRustBridgeWasmWireBase([FutureOr<T>? module]);
}

class JS {
  const JS([String? name]);
}

class _Anonymous {
  const _Anonymous();
}

const anonymous = _Anonymous();

dynamic eval(String script) => throw UnimplementedError();

/// A JS function that returns a Promise to a WASM module.
///
/// ## Enabling cross-origin isolation
/// Rust WASM modules do not work without cross-origin isolation.
abstract class WasmModule {
  Object call([String? moduleName]);

  /// Create a new WASM module initializer that is bound to the specified binary.
  WasmModule bind(dynamic thisArg, String moduleName);

  static Future<T> cast<T extends WasmModule>(FutureOr<WasmModule> module) {
    return Future.value(module).then((module) => module as T);
  }

  /// Initialize a [WasmModule] with the specified kind of [Modules].
  static FutureOr<WasmModule> initialize(
          {required Modules kind, WasmModule Function()? module}) =>
      throw UnimplementedError();
}

/// Currently supported modes of module initialization.
///
/// Advanced users may wish to inherit this class and override [initializeModule]
/// to provide their own initialization process.
abstract class Modules {
  const Modules();

  /// Initialize a `wasm_bindgen` module built with the `-t no-modules` flag.
  ///
  /// The expected output is a file named `$root.js` and the accompanying
  /// WASM binary named `${root}_bg.wasm`.
  const factory Modules.noModules({required String root}) =
      _WasmBindgenNoModules;

  /// How a WASM module is brought into Dart's scope and initialized.
  ///
  /// Override this method to define custom initialization processes.
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module);
}

class _WasmBindgenNoModules extends Modules {
  final String root;
  const _WasmBindgenNoModules({required this.root});

  @override
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module) =>
      throw UnimplementedError();
}
