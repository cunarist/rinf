"""Automation scripts for Rinf project tasks."""

import logging
import os
import subprocess
from pathlib import Path
from sys import argv

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO, format="%(message)s")

ROOT_DIR = Path(__file__).parent.parent
COMMAND_ARG = 1


def run_subprocess(command: str, cwd: Path) -> None:
    """Execute a subprocess command in the specified directory."""
    subprocess.run(command, cwd=cwd, check=True, shell=True)


def replace_text_once(file_path: Path, before: str, after: str) -> None:
    """Replace the first occurrence of text in a file."""
    with file_path.open(encoding="utf8") as file:
        content: str = file.read()
    content = content.replace(before, after, 1)
    with file_path.open(mode="w", encoding="utf8") as file:
        file.write(content)


def search_all_files(directory: Path) -> list[Path]:
    """Recursively search for all files in a directory."""
    all_files: list[Path] = []
    for root, _, files in os.walk(directory):
        root_path = Path(root)
        all_files.extend(root_path / file for file in files)
    return all_files


def update_cargokit() -> None:
    """Update the CargoKit dependency using git subtree."""
    logger.info("Updating CargoKit...")
    run_subprocess(
        (
            "git subtree pull"
            " --prefix flutter_package/cargokit"
            " https://github.com/irondash/cargokit.git"
            " main"
        ),
        ROOT_DIR,
    )


def prepare_test_app() -> None:
    """Set up a test Flutter app for testing purposes."""
    # Prevent side effects.
    file_path = ROOT_DIR / ".gitignore"
    with file_path.open(encoding="utf8") as file:
        content: str = file.read()
    content += "\n/test_app/"
    with file_path.open(mode="w", encoding="utf8") as file:
        file.write(content)

    # Initialize a Flutter app.
    run_subprocess(
        "flutter create test_app",
        ROOT_DIR,
    )
    run_subprocess(
        "dart pub add rinf --path=../flutter_package",
        ROOT_DIR / "test_app",
    )
    run_subprocess(
        "rinf template",
        ROOT_DIR / "test_app",
    )

    # Use repository Cargo workspace.
    (ROOT_DIR / "test_app" / "Cargo.toml").unlink()

    # Enable the web target, since it's not enabled by default.
    crate_path = ROOT_DIR / "test_app" / "native" / "hub"
    replace_text_once(
        crate_path / "Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_once(
        crate_path / "Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )
    for file_path in search_all_files(crate_path / "src"):
        replace_text_once(
            file_path,
            "// use tokio_with_wasm::alias as tokio;",
            "use tokio_with_wasm::alias as tokio;",
        )

    # Update workspace members.
    replace_text_once(
        ROOT_DIR / "Cargo.toml",
        "flutter_package/example/native/*",
        "test_app/native/*",
    )


def prepare_user_app() -> None:
    """Set up a user Flutter app with Rinf from the public package registry."""
    # Prevent side effects.
    file_path = ROOT_DIR / ".gitignore"
    with file_path.open(encoding="utf8") as file:
        content: str = file.read()
    content += "\n/user_app/"
    with file_path.open(mode="w", encoding="utf8") as file:
        file.write(content)

    # Initialize a Flutter app.
    run_subprocess(
        "flutter create user_app",
        ROOT_DIR,
    )
    run_subprocess(
        "flutter pub add rinf",
        ROOT_DIR / "user_app",
    )
    run_subprocess(
        "rinf template",
        ROOT_DIR / "user_app",
    )

    # Use repository Cargo workspace.
    (ROOT_DIR / "user_app" / "Cargo.toml").unlink()

    # Enable the web target, since it's not enabled by default.
    crate_path = ROOT_DIR / "user_app" / "native" / "hub"
    replace_text_once(
        crate_path / "Cargo.toml",
        "# tokio_with_wasm",
        "tokio_with_wasm",
    )
    replace_text_once(
        crate_path / "Cargo.toml",
        "# wasm-bindgen",
        "wasm-bindgen",
    )
    for file_path in search_all_files(crate_path / "src"):
        replace_text_once(
            file_path,
            "// use tokio_with_wasm::alias as tokio;",
            "use tokio_with_wasm::alias as tokio;",
        )

    # Update workspace members.
    replace_text_once(
        ROOT_DIR / "Cargo.toml",
        "flutter_package/example/native/*",
        "user_app/native/*",
    )
    replace_text_once(
        ROOT_DIR / "Cargo.toml",
        'rinf = { path = "rust_crate" }',
        "",
    )


def prepare_example_app() -> None:
    """Generate code for the example Flutter app."""
    run_subprocess(
        "rinf gen",
        ROOT_DIR / "flutter_package" / "example",
    )


def run_command() -> None:
    """Execute the automation command specified in command line arguments."""
    if len(argv) < COMMAND_ARG + 1:
        logger.error("Automation option is not provided")
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
            logger.error("No such option for automation is available")


if __name__ == "__main__":
    run_command()
