import 'bridge_engine/exports.dart';
import 'bridge_generated.dart';

final api = BridgeImpl.wasm(
  WasmModule.initialize(kind: const Modules.noModules(root: 'pkg/hub')),
);
