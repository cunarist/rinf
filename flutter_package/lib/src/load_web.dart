// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js' as js;
import 'dart:html';
import 'dart:async';

String? jsLibPath;

void setJsLibPath(String path) {
  jsLibPath = path;
}

bool wasAlreadyLoaded = false;
js.JsObject rinfBindingsObject = js.context['rinfBindings'];
js.JsObject wasmBindingsObject = js.context['wasmBindings'];

Future<void> loadJsFile() async {
  // When Dart performs hot restart,
  // the `rinfBindings` JavaScript object is already defined
  // as a global JavaScript variable.
  wasAlreadyLoaded = js.context.hasProperty('rinfBindings');

  // Stop loading if it already has been done.
  if (wasAlreadyLoaded) {
    return;
  }

  // Create the namespace JavaScript object.
  // This namespace object is used by Rust
  // to call functions defined in Dart.
  js.context['rinfBindings'] = js.JsObject.jsify({});

  // Prepare to await for the module load.
  final loadCompleter = Completer<void>();
  rinfBindingsObject['completeRinfLoad'] = loadCompleter.complete;

  // Flutter app doesn't always have the top-level path of the domain.
  // Sometimes, the flutter app might be placed in a lower path.
  // This variable includes domain and the base path.
  final baseHref = Uri.base;

  // Use the default JavaScript path unless provided.
  final path = jsLibPath ?? 'pkg/hub.js';

  final fullUrl = baseHref.resolve(path);
  final scriptElement = ScriptElement();
  scriptElement.type = 'module';
  scriptElement.innerHtml = '''
import init, * as wasmBindings from "$fullUrl";
globalThis.wasmBindings = wasmBindings;
await init();
rinfBindings.completeRinfLoad();
delete rinfBindings.completeRinfLoad;
''';
  document.head!.append(scriptElement);

  await loadCompleter.future;
}
