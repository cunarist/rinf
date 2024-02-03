// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js' as js;
import 'dart:html';
import 'dart:async';

var wasAlreadyLoaded = false;

Future<void> loadJsFile() async {
  if (js.context.hasProperty("rinf")) {
    // When Dart performs hot restart,
    // the `rinf` object is already defined
    // as a global JavaScript variable.
    wasAlreadyLoaded = true;
    return;
  }

  final loadCompleter = Completer<void>();
  js.context['completeRinfLoad'] = loadCompleter.complete;

  final scriptElement = ScriptElement();
  scriptElement.type = "module";
  scriptElement.innerHtml = '''
import init, * as wasmBindings from "/pkg/hub.js";
await init();
window.rinf = { ...wasmBindings };
completeRinfLoad();
delete window.completeRinfLoad;
''';
  document.head!.append(scriptElement);

  await loadCompleter.future;
}
