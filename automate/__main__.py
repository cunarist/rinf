import os
import sys
from typing import Any
import toml
from PIL import Image, ImageDraw


def exit():
    print("")
    sys.exit()


def merge_dicts(d1: dict[Any, Any], d2: dict[Any, Any]) -> dict[Any, Any]:
    new: dict[Any, Any] = dict()
    for k in d2.keys():
        if k in d1.keys():
            if isinstance(d1[k], dict) and isinstance(d2[k], dict):
                new[k] = merge_dicts(d1[k], d2[k])
            else:
                new[k] = d1[k]
        else:
            new[k] = d2[k]
    return new


def process_icon(image: Image.Image, roundness: float, scale: float) -> Image.Image:
    round_percent = roundness * 100
    radius = int(round_percent / 100 * image.width)
    width, height = image.size

    mask_image = Image.new("L", (width, height), (0))
    image_draw = ImageDraw.ImageDraw(mask_image)
    image_draw.rounded_rectangle(
        [(0, 0), (image.width, image.height)], radius, fill=(255)
    )

    rounded_image = image.convert("RGBA")
    rounded_image.putalpha(mask_image)

    new_width, new_height = int(width * scale), int(height * scale)
    scaled_image = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    resized_image = rounded_image.resize(
        (new_width, new_height), resample=Image.LANCZOS
    )
    scaled_image.paste(
        resized_image,
        (
            int((width - new_width) / 2),
            int((height - new_height) / 2),
        ),
    )
    return scaled_image


print("")

if len(sys.argv) == 1:
    print("Automation option is not provided.")

elif sys.argv[1] == "app-naming":
    # Qusestion
    app_name = input("Enter the app name. (E.g. My App): ")
    domain = input("Enter domain name. (E.g. com.mycompany): ")
    confirm = input("Are you sure? You cannot change this later. (y/n): ")

    # Check confirmation
    if confirm != "y":
        exit()

    # Set the app name
    lowercase_app_name = app_name.lower().replace(" ", "")
    for path, subdirs, files in os.walk("./"):
        for name in files:
            if ".py" in name:
                continue
            filepath = os.path.join(path, name)
            try:
                with open(filepath, mode="r", encoding="utf8") as file:
                    content = file.read()
                modified = content
                modified = modified.replace("someappname", lowercase_app_name)
                modified = modified.replace("Some App Name", app_name)
                modified = modified.replace("com.example", domain)
                if modified != content:
                    with open(filepath, mode="w", encoding="utf8") as file:
                        file.write(modified)
            except UnicodeDecodeError:
                pass

    combined = f"{domain}.{lowercase_app_name}"
    command = f"flutter pub run change_app_package_name:main {combined}"
    os.system(command)

    print("Done! Don't forget to update description in pubspec.yaml file as well.")

elif sys.argv[1] == "config-filling":
    # Scan
    filepath = "./android/local.properties"
    lines = []
    if os.path.isfile(filepath):
        with open(filepath, mode="r", encoding="utf8") as file:
            lines = file.read().splitlines()

    does_exist = False
    for line in lines:
        if line.startswith("ndk.dir"):
            does_exist = True
            break

    if not does_exist:
        lines.append("ndk.dir= # Androkd NDK path on your system")

    filepath = "./android/local.properties"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write("\n".join(lines))
    print("Updated local.properties file with necessary fields.")

    # Scan
    filepath = "./native/.cargo/config.toml"
    original_tree = {}
    if os.path.isfile(filepath):
        with open(filepath, mode="r", encoding="utf8") as file:
            original_tree = toml.load(file)

    # Merge
    filepath = "./native/.cargo/config.toml.template"
    with open(filepath, mode="r", encoding="utf8") as file:
        template_tree = toml.load(file)
    final_tree = merge_dicts(original_tree, template_tree)
    filepath = "./native/.cargo/config.toml"
    with open(filepath, mode="w", encoding="utf8") as file:
        toml.dump(final_tree, file)
    text = "Updated config.toml file with the template file next to it."
    print(text)

    print("Now go ahead and fill out the fields in those files!")

elif sys.argv[1] == "bridge-gen":
    command = "flutter_rust_bridge_codegen"
    command += f" --rust-input ./native/hub/src/bridge/api.rs"
    command += f" --dart-output ./lib/bridge/bridge_generated.dart"
    command += f" --class-name Bridge"
    command += f" --rust-output ./native/hub/src/bridge/bridge_generated.rs"
    command += f" -c ios/Runner/bridge_generated.h"
    command += f" -e macos/Runner/"
    os.system(command)

    filepath = "./native/hub/src/lib.rs"
    with open(filepath, mode="r", encoding="utf8") as file:
        lines = file.readlines()

    for turn, line in enumerate(lines):
        if "AUTO INJECTED BY flutter_rust_bridge" in line:
            lines[turn] = ""

    filepath = "./native/hub/src/lib.rs"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write("".join(lines))

elif sys.argv[1] == "template-update":
    command = "git remote rm template"
    os.system(command)
    command = "git remote add template https://github.com/cunarist/app-template.git"
    os.system(command)
    command = "git fetch --all"
    os.system(command)
    command = "git merge template/main --allow-unrelated-histories"
    os.system(command)

elif sys.argv[1] == "code-quality":
    command = "black ."
    os.system(command)
    command = "dart fix --apply"
    os.system(command)
    path = "./native"
    os.chdir(path)
    command = "cargo clippy --fix --allow-dirty"
    os.system(command)

elif sys.argv[1] == "size-check":
    path = "./native"
    os.chdir(path)
    command = "cargo bloat --release"
    os.system(command)
    if len(sys.argv) == 2:
        print("Platform option is not provided.")
    else:
        path = ".."
        os.chdir(path)
        command = f"flutter build {sys.argv[2]} --analyze-size"
        os.system(command)

elif sys.argv[1] == "icon-gen":
    image = Image.open("./assets/app_icon_full.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./temp/app_icon_windows.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./temp/app_icon_linux.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./temp/app_icon_android.png")

    new_image = process_icon(image, 0.232, 0.8)
    new_image.save("./temp/app_icon_macos.png")

    new_image = process_icon(image, 0, 1)
    new_image.save("./temp/app_icon_ios.png")

    command = "flutter pub run flutter_launcher_icons"
    os.system(command)

elif sys.argv[1] == "translation":
    filepath = "./assets/translations.csv"
    with open(filepath, mode="r", encoding="utf8") as file:
        first_line = file.readline()
    languages = first_line.split(",")[1:]
    languages = [t.strip() for t in languages]

    filepath = "./ios/Runner/Info.plist"
    with open(filepath, mode="r", encoding="utf8") as file:
        lines = file.read().split("\n")
    for turn, line in enumerate(lines):
        if "<key>CFBundleLocalizations</key>" in line:
            array_start_line = turn + 1
            break

    for line_number in range(array_start_line, len(lines)):
        if "</array>" in lines[line_number]:
            array_end_line = line_number

    lines = lines[:array_start_line] + lines[array_end_line + 1 :]

    language_strings = [" " * 8 + f"<string>{l}</string>" for l in languages]
    language_strings.insert(0, " " * 4 + "<array>")
    language_strings.append(" " * 4 + "</array>")
    print(language_strings)
    lines = lines[:array_start_line] + language_strings + lines[array_start_line:]
    final_text = "\n".join(lines)

    filepath = "./ios/Runner/Info.plist"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(final_text)

else:
    print("No such option for automation is available.")

exit()
