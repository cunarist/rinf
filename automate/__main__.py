from os import remove
from pathlib import Path
from subprocess import run
from sys import argv

ROOT_DIR = Path(__file__).parent.parent
COMMAND_ARG = 1


def run_subprocess(command: str, cwd: Path):
    run(command.split(), cwd=cwd, check=True, shell=True)


def replace_text_once(filepath: Path, change_from: str, change_to: str):
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content = content.replace(change_from, change_to, 1)
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)


def update_cargokit():
    print("Updating CargoKit...")
    command = (
        "git subtree pull"
        " --prefix flutter_package/cargokit"
        " https://github.com/irondash/cargokit.git"
        " main"
    )
    run_subprocess(command, ROOT_DIR)


def prepare_test_app():
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/test_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create test_app"
    run_subprocess(command, ROOT_DIR)

    command = "dart pub add rinf --path=../flutter_package"
    run_subprocess(command, ROOT_DIR / "test_app")
    command = "rinf template"
    run_subprocess(command, ROOT_DIR / "test_app")

    # Use repository Cargo workspace.
    remove(ROOT_DIR / "test_app" / "Cargo.toml")

    # Enable the web target, since it's not enabled by default.
    replace_text_once(
        ROOT_DIR / "test_app" / "native" / "hub" / "src" / "lib.rs",
        "// use tokio_with_wasm::alias as tokio;",
        "use tokio_with_wasm::alias as tokio;",
    )
    replace_text_once(
        ROOT_DIR / "test_app" / "native" / "hub" / "Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_once(
        ROOT_DIR / "test_app" / "native" / "hub" / "Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )

    # Update workspace members.
    replace_text_once(
        ROOT_DIR / "Cargo.toml",
        "flutter_package/example/native/*",
        "test_app/native/*",
    )


def prepare_user_app():
    filepath = ".gitignore"
    with open(filepath, mode="r", encoding="utf8") as file:
        content: str = file.read()
    content += "\n/user_app/"
    with open(filepath, mode="w", encoding="utf8") as file:
        file.write(content)

    command = "flutter create user_app"
    run_subprocess(command, ROOT_DIR)

    command = "flutter pub add rinf"
    run_subprocess(command, ROOT_DIR / "user_app")
    command = "rinf template"
    run_subprocess(command, ROOT_DIR / "user_app")

    # Use repository Cargo workspace.
    remove(ROOT_DIR / "user_app" / "Cargo.toml")

    # Enable the web target, since it's not enabled by default.
    replace_text_once(
        ROOT_DIR / "user_app" / "native" / "hub" / "src" / "lib.rs",
        "// use tokio_with_wasm::alias as tokio;",
        "use tokio_with_wasm::alias as tokio;",
    )
    replace_text_once(
        ROOT_DIR / "user_app" / "native" / "hub" / "Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_once(
        ROOT_DIR / "user_app" / "native" / "hub" / "Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )

    # Update workspace members.
    replace_text_once(
        ROOT_DIR / "Cargo.toml",
        "flutter_package/example/native/*",
        "user_app/native/*",
    )
    replace_text_once(
        ROOT_DIR / "Cargo.toml",
        'rinf = { path = "./rust_crate" }',
        "",
    )


def prepare_example_app():
    command = "rinf gen"
    run_subprocess(command, ROOT_DIR / "flutter_package" / "example")


def run_command():
    if len(argv) < COMMAND_ARG + 1:
        print(
            "Automation option is not provided;"
            "\nUse `python automate --help` to see all available operations"
        )
        return

    command = argv[COMMAND_ARG]
    match command:
        case "update-cargokit":
            update_cargokit()
        case "prepare-test-app":
            prepare_test_app()
        case "prepare-user-app":
            prepare_user_app()
        case "prepare-example-app":
            prepare_example_app()
        case _:
            print("No such option for automation is available")


run_command()
