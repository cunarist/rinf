// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:async';
import 'dart:js_interop';
import 'dart:js_interop_unsafe';
import 'package:web/web.dart';

String? jsLibPath;

void setJsLibPath(String path) {
  jsLibPath = path;
}

bool wasAlreadyLoaded = false;
final rinfBindingsObject = globalContext['rinfBindings'] as JSObject;
final wasmBindingsObject = globalContext['wasmBindings'] as JSObject;

Future<void> loadJsFile() async {
  // When Dart performs hot restart,
  // the `rinfBindings` JavaScript object is already defined
  // as a global JavaScript variable.
  wasAlreadyLoaded = globalContext.hasProperty('rinfBindings'.toJS) as bool;

  // Stop loading if it already has been done.
  if (wasAlreadyLoaded) {
    return;
  }

  // Create the namespace JavaScript object.
  // This namespace object is used by Rust
  // to call functions defined in Dart.
  globalContext['rinfBindings'] = JSObject();

  // Prepare to await the module load.
  final loadCompleter = Completer<void>();
  rinfBindingsObject['completeRinfLoad'] = loadCompleter.complete.jsify();

  // Flutter app doesn't always have the top-level path of the domain.
  // Sometimes, the flutter app might be placed in a lower path.
  // This variable includes domain and the base path.
  final baseHref = Uri.base;

  // Use the default JavaScript path unless provided.
  final path = jsLibPath ?? 'pkg/hub.js';

  final fullUrl = baseHref.resolve(path);
  final scriptElement = HTMLScriptElement();
  scriptElement.type = 'module';
  scriptElement.innerHTML = '''
import init, * as wasmBindings from "$fullUrl";
globalThis.wasmBindings = wasmBindings;
await init();
rinfBindings.completeRinfLoad();
delete rinfBindings.completeRinfLoad;
'''
      .toJS;
  document.head!.append(scriptElement);

  await loadCompleter.future;
}
