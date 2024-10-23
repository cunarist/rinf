// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js' as js;
import 'dart:html';
import 'dart:async';

String? jsLibPath;

void setJsLibPath(String path) {
  jsLibPath = path;
}

bool wasAlreadyLoaded = false;
js.JsObject rinfBindingsObject = getRinfBindingsObject();
js.JsObject wasmBindingsObject = js.context['wasmBindings'];

/// When Dart performs hot restart,
/// the `rinfBindings` JavaScript object is already defined
/// as a global JavaScript variable.
void checkIfAlreadyLoaded() {
  wasAlreadyLoaded = js.context.hasProperty('rinfBindings');
}

/// Create the namespace JavaScript object.
/// This namespace object is used by Rust
/// to call functions defined in Dart.
js.JsObject getRinfBindingsObject() {
  // Create a new `rinfBindings` JavaScript object if not present.
  // Otherwise, return the existing one.
  final js.JsObject jsObject;
  if (wasAlreadyLoaded) {
    jsObject = js.context['rinfBindings'];
  } else {
    jsObject = js.JsObject.jsify({});
    js.context['rinfBindings'] = jsObject;
  }
  return jsObject;
}

Future<void> loadJsFile() async {
  if (wasAlreadyLoaded) {
    return;
  }

  final loadCompleter = Completer<void>();
  js.context['completeRinfLoad'] = loadCompleter.complete;

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
completeRinfLoad();
delete window.completeRinfLoad;
''';
  document.head!.append(scriptElement);

  await loadCompleter.future;
}
