import 'src/helpers.dart';
import 'src/message.dart';
import 'src/config.dart';

Future<void> main(List<String> args) async {
  if (args.length == 0) {
    print("No operation is provided.");
    print("Use `rinf --help` to see all available operations.");
    return;
  }

  final rinfConfig = await loadVerifiedRinfConfig("pubspec.yaml");

  switch (args[0]) {
    case "config":
      print(rinfConfig);
      break;
    case "template":
      if (args.contains("--bridge") || args.contains("-b")) {
        await applyRustTemplate(
          onlyBridge: true,
          messageConfig: rinfConfig.message,
        );
      } else {
        await applyRustTemplate(messageConfig: rinfConfig.message);
      }
      break;
    case "message":
      if (args.contains("--watch") || args.contains("-w")) {
        await watchAndGenerateMessageCode(messageConfig: rinfConfig.message);
      } else {
        await generateMessageCode(messageConfig: rinfConfig.message);
      }
      break;
    case "wasm":
      if (args.contains("--release") || args.contains("-r")) {
        await buildWebassembly(isReleaseMode: true);
      } else {
        await buildWebassembly();
      }
      break;
    case "--help":
    case "-h":
      print("Usage: rinf [arguments]");
      print("Arguments:");
      print("  -h, --help        Shows this usage information.");
      print("  config            Shows current Rinf configuration"
          "\n                    resolved from `pubspec.yaml`"
          "\n                    with defaults applied.");
      print("  template          Applies Rust template to current project.");
      print("    -b, --bridge    Only applies `bridge` Rust module.");
      print("  message           Generates message code from `.proto` files.");
      print("    -w, --watch     Continuously watches `.proto` files.");
      print("  wasm              Builds webassembly module.");
      print("    -r, --release   Builds in release mode.");
    default:
      print("No such operation is available.");
      print("Use `rinf --help` to see all available operations.");
  }
}
