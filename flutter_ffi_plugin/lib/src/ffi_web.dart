import 'bridge/engine/exports.dart';
import 'bridge/generated.dart';

final api = BridgeImpl.wasm(
  WasmModule.initialize(kind: const Modules.noModules(root: 'pkg/hub')),
);
