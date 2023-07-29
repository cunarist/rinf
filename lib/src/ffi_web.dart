import 'bridge_engine/exports.dart';
import 'bridge_web_generated.dart';

final api = BridgeWebImpl.wasm(
  WasmModule.initialize(kind: const Modules.noModules(root: 'pkg/hub')),
);
