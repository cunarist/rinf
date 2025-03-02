import os
import sys
import time


# TODO: Use `Path` to manage paths.


def replace_text_in_file(filepath: str, change_from: str, change_to: str):
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content = content.replace(change_from, change_to)
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)


if len(sys.argv) == 1:
    print("Automation option is not provided.")
    print("Use `python automate --help` to see all available operations.")

elif sys.argv[1] == "cargokit-update":
    print("Updating CargoKit...")
    command = "git subtree pull"
    command += " --prefix flutter_package/cargokit"
    command += " https://github.com/irondash/cargokit.git"
    command += " main"
    os.system(command)

elif sys.argv[1] == "prepare-test-app":
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/test_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create test_app"
    os.system(command)

    os.chdir("./test_app/")

    command = "dart pub add \"rinf:{'path':'../flutter_package'}\""
    os.system(command)
    command = "rinf template"
    while os.system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        time.sleep(60)

    os.remove("Cargo.toml")

    # Enable the web target, since it's not enabled by default.
    replace_text_in_file(
        "native/hub/src/lib.rs",
        "// use tokio_with_wasm::alias as tokio;",
        "use tokio_with_wasm::alias as tokio;",
    )
    replace_text_in_file(
        "native/hub/Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_in_file(
        "native/hub/Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )

    os.chdir("../")

    replace_text_in_file(
        "Cargo.toml",
        "flutter_package/example/native/*",
        "test_app/native/*",
    )

elif sys.argv[1] == "prepare-user-app":
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

    os.remove("Cargo.toml")

    # Enable the web target, since it's not enabled by default.
    replace_text_in_file(
        "native/hub/src/lib.rs",
        "// use tokio_with_wasm::alias as tokio;",
        "use tokio_with_wasm::alias as tokio;",
    )
    replace_text_in_file(
        "native/hub/Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_in_file(
        "native/hub/Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )

    os.chdir("../")

    replace_text_in_file(
        "Cargo.toml",
        "flutter_package/example/native/*",
        "user_app/native/*",
    )
    replace_text_in_file(
        "Cargo.toml",
        'rinf = { path = "./rust_crate" }',
        "",
    )

elif sys.argv[1] == "prepare-example-app":
    os.chdir("./flutter_package/example")

    command = "rinf message"
    while os.system(command) != 0:
        # Retry the command in case of failure,
        # possibly due to GitHub API rate limiting
        # associated with the 'protoc_prebuilt' crate.
        time.sleep(60)

else:
    print("No such option for automation is available.")
