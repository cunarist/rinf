import os
import sys


def exit():
    print("")
    sys.exit()


print("")

if len(sys.argv) == 1:
    print("Automation option is not provided.")

elif sys.argv[1] == "bridge-gen":
    command = "flutter_rust_bridge_codegen"
    command += f" --rust-input ./example/native/hub/src/bridge/api.rs"
    command += f" --rust-output ./example/native/hub/src/bridge/bridge_generated.rs"
    command += f" --dart-output ./lib/src/bridge_generated.dart"
    command += f" --dart-decl-output ./lib/src/bridge_definitions.dart"
    command += f" --class-name Bridge"
    command += f" -c ./ios/Classes/bridge_generated.h"
    command += f" -e ./macos/Classes/"
    command += f" --wasm"
    os.system(command)

    filepath = "./example/native/hub/src/lib.rs"
    with open(filepath, mode="r", encoding="utf8") as file:
        lines = file.readlines()
    for turn, line in enumerate(lines):
        if "AUTO INJECTED BY flutter_rust_bridge" in line:
            lines[turn] = ""
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write("".join(lines))

else:
    print("No such option for automation is available.")

exit()
