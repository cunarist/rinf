import os
import sys
import shutil
from typing import Any
import toml


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
    # Scan .env file
    filepath = "./.env"
    written_pairs = {}
    if os.path.isfile(filepath):
        with open(filepath, mode="r", encoding="utf8") as file:
            lines = file.read().splitlines()
        lines = [x.strip().split("=", 1) for x in lines if "=" in x]
        written_pairs = dict(lines)

    # Update .env file
    filepath = "./.env.template"
    with open(filepath, mode="r", encoding="utf8") as file:
        lines = file.read().splitlines()
    for turn, line in enumerate(lines):
        if "=" not in line:
            continue
        item_key = line.strip().split("=", 1)[0]
        if item_key in written_pairs.keys():
            item_value = written_pairs[item_key]
            lines[turn] = f"{item_key}={item_value}"
    text = "".join(lines)
    filepath = ".env"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(text)
    print("Updated .env file from .env.template file in ./ folder.")

    # Scan config.toml file
    filepath = "./native/.cargo/config.toml"
    original_tree = {}
    if os.path.isfile(filepath):
        with open(filepath, mode="r", encoding="utf8") as file:
            original_tree = toml.load(file)

    # Update config.toml file
    filepath = "./native/.cargo/config.toml.template"
    with open(filepath, mode="r", encoding="utf8") as file:
        template_tree = toml.load(file)
    final_tree = merge_dicts(original_tree, template_tree)
    filepath = "./native/.cargo/config.toml"
    with open(filepath, mode="w", encoding="utf8") as file:
        toml.dump(final_tree, file)
    text = "Updated config.toml file from config.toml.template file"
    text += " in ./native/.cargo folder."
    print(text)

elif sys.argv[1] == "bridge-gen":
    # Clear bridge folder
    folderpath = f"./lib/bridge"
    if os.path.exists(folderpath) and os.path.isdir(folderpath):
        shutil.rmtree(folderpath)

    # Generate ffi.dart file
    os.makedirs(f"./lib/bridge", exist_ok=True)

    filepath = "./automate/template/ffi.dart.txt"
    with open(filepath, mode="r", encoding="utf8") as file:
        template_text = file.read()

    crate_name = "bridge"
    output_text = template_text
    output_text = output_text.replace("[[CRATE]]", crate_name)
    class_name = crate_name.replace("_", " ").title().replace(" ", "")
    output_text = output_text.replace("[[CLASS]]", class_name)

    filepath = f"./lib/bridge/ffi.dart"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(output_text)

    command = "flutter_rust_bridge_codegen"
    command += f" -r ./native/bridge/src/api.rs"
    command += f" -d ./lib/bridge/bridge_generated.dart"
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

else:
    print("No such option for automation is available.")

exit()
