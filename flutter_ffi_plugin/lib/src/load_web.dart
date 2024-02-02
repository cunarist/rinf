// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js' as js;
import 'dart:html';
import 'dart:async';

var isAlreadyPrepared = false;

Future<void> loadJsFile() async {
  if (js.context.hasProperty("rinf")) {
    // When Dart performs hot restart,
    // the `rinf` object is already defined
    // as a global JavaScript variable.
    isAlreadyPrepared = true;
    return;
  }

  final jsObject = js.JsObject.jsify({});
  js.context['rinf'] = jsObject;

  final loadCompleter = Completer<void>();
  jsObject['load_complete'] = loadCompleter.complete;

  final scriptElement = ScriptElement();
  scriptElement.type = "module";
  scriptElement.innerHtml = '''
import init, * as wasm from "/pkg/hub.js";
await init();
window.rinf = { ...rinf, ...wasm };
rinf.load_complete();
delete rinf.load_complete;
''';
  document.head!.append(scriptElement);

  await loadCompleter.future;
}
