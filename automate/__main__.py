import os
import sys
import json
from PIL import Image, ImageDraw
import tomlkit


def exit():
    print("")
    sys.exit()


def process_icon(image: Image.Image, roundness: float, scale: float) -> Image.Image:
    round_percent = roundness * 100
    radius = int(round_percent / 100 * image.width)
    width, height = image.size

    mask_image = Image.new("L", (width, height), (0))
    image_draw = ImageDraw.ImageDraw(mask_image)
    image_draw.rounded_rectangle(
        ((0.0, 0.0), (float(image.width), float(image.height))), radius, fill=(255)
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


def merge_toml_files(filepath: str, template_filepath: str):
    # Read the template file
    with open(template_filepath) as template_file:
        template_content = template_file.read()
    # Parse the template content
    template = tomlkit.parse(template_content)
    # Check if the config file exists
    if os.path.isfile(filepath):
        # Read the existing TOML file
        with open(filepath) as config_file:
            config_content = config_file.read()
        # Parse the existing TOML content
        config = tomlkit.parse(config_content)
    else:
        # If the config file doesn't exist, use an empty TOML document
        config = tomlkit.document()
    Table = type(tomlkit.table())

    # Helper function to recursively merge tables
    def merge_docs(existing_doc, new_doc):
        for key, value in new_doc.items():
            if key in existing_doc:
                # Check if the value is a table (nested structure)
                if isinstance(value, Table) and isinstance(existing_doc[key], Table):
                    # If both values are TOML document, recursively merge them
                    merge_docs(existing_doc[key], value)
            else:
                # If the key doesn't exist, add a new key-value pair and preserve the comments
                existing_doc[key] = value

    # Merge the template into the config preserving comments
    merge_docs(config, template)
    # Serialize the merged TOML back to string
    merged_content = tomlkit.dumps(config)
    # Write the merged TOML to the config file
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, "w") as output_file:
        output_file.write(merged_content)


def merge_properties_files(filepath: str, template_filepath: str):
    # Read the template file
    with open(template_filepath) as template_file:
        template_content = template_file.read().split("\n")
    # Check if the config file exists
    if os.path.isfile(filepath):
        # Read the existing properties file
        with open(filepath) as config_file:
            config_content = config_file.read().split("\n")
    else:
        # If the config file doesn't exist, use an empty content list
        config_content = []

    def merge_properties_content(existing_content: list, new_content: list):
        merged_content = existing_content.copy()
        existing_keys = {
            line.split("=", 1)[0].strip()
            for line in existing_content
            if line and not line.startswith("#")
        }
        for line in new_content:
            line = line.strip()
            if line and not line.startswith("#"):
                key, value = line.split("=", 1)
                if key not in existing_keys:
                    merged_content.append(f"{key}={value}")
        return merged_content

    merged_content = merge_properties_content(config_content, template_content)
    # Write the merged properties to the config file
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, "w") as output_file:
        output_file.write("\n".join(merged_content))


def merge_json_files(filepath: str, template_filepath: str):
    # Read the template file
    with open(template_filepath) as template_file:
        template_content = template_file.read()
    # Parse the template JSON
    template = json.loads(template_content)
    # Check if the config file exists
    if os.path.isfile(filepath):
        # Read the existing JSON file
        with open(filepath) as config_file:
            config_content = config_file.read()
        # Parse the existing JSON
        config = json.loads(config_content)
    else:
        # If the config file doesn't exist, use an empty dict
        config = {}

    # Helper function to recursively merge JSON objects
    def merge_objects(existing_obj, new_obj):
        for key, value in new_obj.items():
            if (
                key in existing_obj
                and isinstance(value, dict)
                and isinstance(existing_obj[key], dict)
            ):
                # If both values are dicts, recursively merge them
                merge_objects(existing_obj[key], value)
            else:
                # Otherwise, update the value
                existing_obj[key] = value

    # Merge the template into the config
    merge_objects(config, template)
    # Serialize the merged JSON back to a string
    merged_content = json.dumps(config, indent=4)
    # Write the merged JSON to the config file
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, "w") as output_file:
        output_file.write(merged_content)


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
    command = f"dart run change_app_package_name:main {combined}"
    os.system(command)

    print("Done! Don't forget to update description in pubspec.yaml file as well.")

elif sys.argv[1] == "config-filling":
    # Android
    filepath = "./android/local.properties"
    merge_properties_files(filepath, f"{filepath}.template")
    print(f"Updated {filepath}")

    # Rust
    filepath = "./native/.cargo/config.toml"
    merge_toml_files(filepath, f"{filepath}.template")
    print(f"Updated {filepath}")

    # Visual Studio Code
    filepath = "./.vscode/settings.json"
    merge_json_files(filepath, f"{filepath}.template")
    print(f"Updated {filepath}")

    print("")
    print("Now go ahead and manually fill in those files!")

elif sys.argv[1] == "bridge-gen":
    command = "flutter_rust_bridge_codegen"
    command += f" --rust-input ./native/hub/src/bridge/api.rs"
    command += f" --dart-output ./lib/bridge/bridge_generated.dart"
    command += f" --class-name Bridge"
    command += f" --rust-output ./native/hub/src/bridge/bridge_generated.rs"
    command += f" -c ios/Runner/bridge_generated.h"
    command += f" -e macos/Runner/"
    command += f" --wasm"
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
    command = "python -m black ."
    os.system(command)
    command = "dart fix --apply"
    os.system(command)
    path = "./native"
    os.chdir(path)
    command = "cargo clippy --fix --allow-dirty"
    os.system(command)

elif sys.argv[1] == "size-check":
    command = "cargo install cargo-bloat"
    os.system(command)
    path = "./native"
    os.chdir(path)
    command = "cargo bloat --release -n 50"
    os.system(command)
    if len(sys.argv) == 2:
        print("")
        print("Platform option is not provided.")
    else:
        path = ".."
        os.chdir(path)
        command = f"flutter build {sys.argv[2]} --analyze-size"
        os.system(command)

elif sys.argv[1] == "icon-gen":
    os.makedirs("./build/app_icons", exist_ok=True)

    image = Image.open("./assets/app_icon_full.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./build/app_icons/windows.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./build/app_icons/linux.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./build/app_icons/android.png")

    new_image = process_icon(image, 0.232, 0.8)
    new_image.save("./build/app_icons/macos.png")

    new_image = process_icon(image, 0, 1)
    new_image.save("./build/app_icons/ios.png")

    new_image = process_icon(image, 0.232, 1)
    new_image.save("./build/app_icons/web.png")

    command = "dart run flutter_launcher_icons"
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

    array_start_line = 0
    for turn, line in enumerate(lines):
        if "<key>CFBundleLocalizations</key>" in line:
            array_start_line = turn + 1
            break

    array_end_line = 0
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

elif sys.argv[1] == "serve-web":
    command = "dart run flutter_rust_bridge:serve"
    command += f" --crate=./native/hub/"
    os.system(command)

else:
    print("No such option for automation is available.")

exit()
