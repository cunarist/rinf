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

    print("Done! Don't forget to update description in pubspec.yaml file as well.")

elif sys.argv[1] == "config-filling":
    # Scan
    filepath = "./android/local.properties"
    written_pairs = {}
    if os.path.isfile(filepath):
        with open(filepath, mode="r", encoding="utf8") as file:
            lines = file.read().splitlines()
        lines = [x.strip().split("=", 1) for x in lines if "=" in x]
        written_pairs = dict(lines)

    # Merge
    filepath = "./android/local.properties.template"
    with open(filepath, mode="r", encoding="utf8") as file:
        lines = file.read().splitlines()
    for turn, line in enumerate(lines):
        if "=" not in line:
            continue
        item_key = line.strip().split("=", 1)[0]
        item_value = line.strip().split("=", 1)[1]
        if item_key not in written_pairs.keys():
            written_pairs[item_key] = item_value
    text = "\n".join([f"{k}={v}" for k, v in written_pairs.items()])
    filepath = "./android/local.properties"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(text)
    print("Updated local.properties file from the template file in ./android folder.")

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
    text = "Updated config.toml file from the template file"
    text += " in ./native/.cargo folder."
    print(text)

    print("Now go ahead and fill out the fields in those files!")

elif sys.argv[1] == "bridge-gen":
    command = "flutter_rust_bridge_codegen"
    command += f" -r ./native/bridge/src/api.rs"
    command += f" -d ./lib/bridge/bridge_generated.dart"
    command += f" -c ios/Runner/bridge_generated.h"
    command += f" -e macos/Runner/"
    os.system(command)

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

    new_image = process_icon(image, 0, 1)
    new_image.save("./temp/app_icon_android.png")

    new_image = process_icon(image, 0.232, 0.8)
    new_image.save("./temp/app_icon_macos.png")

    new_image = process_icon(image, 0, 1)
    new_image.save("./temp/app_icon_ios.png")

    command = "flutter pub run flutter_launcher_icons"
    os.system(command)

else:
    print("No such option for automation is available.")

exit()
