import os
import sys
import time


if len(sys.argv) == 1:
    print("Automation option is not provided.")
    print("Use `python automate --help` to see all available operations.")

elif sys.argv[1] == "cargokit-update":
    print("Updating CargoKit...")
    command = "git subtree pull"
    command += " --prefix flutter_ffi_plugin/cargokit"
    command += " https://github.com/irondash/cargokit.git"
    command += " main"
    os.system(command)

elif sys.argv[1] == "create-test-app":
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/test_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create test_app"
    os.system(command)

    os.chdir("./test_app/")

    command = "dart pub add \"rinf:{'path':'../flutter_ffi_plugin'}\""
    os.system(command)
    command = "rinf template"
    while os.system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        time.sleep(60)

    os.remove("Cargo.toml")

    os.chdir("../")

    filepath = "Cargo.toml"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content = content.replace(
        "flutter_ffi_plugin/example/native/*",
        "test_app/native/*",
    )
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

elif sys.argv[1] == "create-user-app":
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/user_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create user_app"
    os.system(command)

    os.chdir("./user_app/")

    command = "flutter pub add rinf"
    os.system(command)
    command = "rinf template"
    while os.system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        time.sleep(60)

    os.chdir("../")

    filepath = "Cargo.toml"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content = content.replace(
        "flutter_ffi_plugin/example/native/*",
        "user_app/native/*",
    )
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

else:
    print("No such option for automation is available.")
