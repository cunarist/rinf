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
  wasAlreadyLoaded = globalContext.hasProperty('rinfBindings'.toJS).toDart;

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
  rinfBindingsObject['completeRinfLoad'] = (() {
    loadCompleter.complete();
  }).toJS;

  // Get the domain and the base path.
  // Flutter app doesn't always have the top-level path of the domain.
  // Sometimes, the flutter app might be placed in a lower path.
  final baseHref = Uri.base;
  final path = jsLibPath ?? 'pkg/hub.js';
  final fullUrl = baseHref.resolve(path);

  // Insert the script element into the document head.
  // This will load the webassembly module.
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

  // Await for the module to load.
  await loadCompleter.future;
}
