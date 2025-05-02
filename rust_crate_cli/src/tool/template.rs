use crate::tool::{
  CaptureError, CleanFileName, DART_BIN, RinfConfig, SetupError,
  generate_dart_code,
};
use include_dir::{Dir, include_dir};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;
use std::process::Command;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

/// Creates new folders and files in an existing Flutter project folder.
pub fn apply_rust_template(
  root_dir: &Path,
  rinf_config: &RinfConfig,
) -> Result<(), SetupError> {
  if root_dir.join("native").is_dir() {
    return Err(SetupError::TemplateApplied);
  }

  // Copy basic folders needed for Rust to work.
  dump_template(root_dir)?;

  // Modify `.gitignore`.
  update_gitignore(root_dir)?;

  // Modify `README.md`.
  update_readme(root_dir)?;

  // Add Dart dependencies.
  Command::new(DART_BIN)
    .args(["pub", "add", "meta"])
    .output()?
    .capture_err()?;
  Command::new(DART_BIN)
    .args(["pub", "add", "tuple"])
    .output()?
    .capture_err()?;

  // Modify `lib/main.dart`
  update_main_dart(root_dir)?;

  // Format Rust code.
  Command::new("cargo")
    .args(["fmt"])
    .output()?
    .capture_err()?;

  // Generate Dart code.
  generate_dart_code(root_dir, rinf_config)?;

  Ok(())
}

/// Recursively extracts the embedded `TEMPLATE_DIR` to `dest_path`
fn dump_template(dest_path: &Path) -> Result<(), SetupError> {
  for entry in TEMPLATE_DIR.entries() {
    dump_entry(dest_path, entry)?;
  }
  Ok(())
}

fn dump_entry(
  dest_path: &Path,
  entry: &include_dir::DirEntry,
) -> Result<(), SetupError> {
  match entry {
    include_dir::DirEntry::Dir(dir) => {
      let dir_path = dest_path.join(dir.path());
      create_dir_all(&dir_path)?;
      for sub_entry in dir.entries() {
        dump_entry(dest_path, sub_entry)?;
      }
    }
    include_dir::DirEntry::File(file) => {
      let mut file_path = dest_path.join(file.path());
      let file_name = file_path.clean_file_name()?;
      // The existence of files like `Cargo.toml` prevents us
      // from including the folder in the crate.
      // That's why we add the `.template` extension to them.
      let clean_name =
        file_name.strip_suffix(".template").unwrap_or(&file_name);
      file_path.set_file_name(clean_name);
      if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
      }
      write(file_path, file.contents())?;
    }
  }
  Ok(())
}

fn update_gitignore(root_dir: &Path) -> Result<(), SetupError> {
  let gitignore_path = root_dir.join(".gitignore");
  let mut content = read_to_string(&gitignore_path).unwrap_or_default();
  if !content.contains("# Rust related") {
    content.push_str("\n# Rust related\n/.cargo/\n/target/\n");
  }
  if !content.contains("# Generated signals") {
    content.push_str(
      r#"
# Generated signals
/lib/src/bincode
/lib/src/serde
/lib/src/bindings
"#,
    );
  }
  write(gitignore_path, content)?;
  Ok(())
}

fn update_readme(root_dir: &Path) -> Result<(), SetupError> {
  let readme_path = root_dir.join("README.md");
  let mut content = read_to_string(&readme_path).unwrap_or_default();
  let guide_section = "## Using Rust Inside Flutter";

  if !content.contains(guide_section) {
    content = content.trim_end().to_owned();
    content.push_str("\n\n");
    content.push_str(guide_section);
    content.push_str("\n\n");
    content.push_str(
      r#"This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rinf](https://pub.dev/packages/rinf) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install)
and [Rust toolchain](https://www.rust-lang.org/tools/install)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```shell
rustc --version
flutter doctor
```

You also need to have the CLI tool for Rinf ready.

```shell
cargo install rinf_cli
```

Signals sent between Dart and Rust are implemented using signal attributes.
If you've modified the signal structs, run the following command
to generate the corresponding Dart classes:

```shell
rinf gen
```

Now you can run and build this app just like any other Flutter projects.

```shell
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rinf's [documentation](https://rinf.cunarist.com).
"#,
    );
  }

  write(readme_path, content)?;
  Ok(())
}

fn update_main_dart(root_dir: &Path) -> Result<(), SetupError> {
  let main_path = root_dir.join("lib/main.dart");
  if main_path.exists() {
    Command::new(DART_BIN)
      .args(["format", "lib/main.dart"])
      .output()?
      .capture_err()?;
    let mut content = read_to_string(&main_path)?;
    if !content.contains("messages/all.dart") {
      content = content.replacen(
        "import",
        "import 'package:rinf/rinf.dart';\
        \nimport 'src/bindings/bindings.dart';\nimport",
        1,
      );
    }
    if !content.contains("initializeRust(assignRustSignal)") {
      content = content.replacen(
        "main() {",
        "main() async {\n    await initializeRust(assignRustSignal);",
        1,
      );
    }
    write(main_path, content)?;
    Command::new(DART_BIN)
      .args(["format", "lib/main.dart"])
      .output()?
      .capture_err()?;
  }
  Ok(())
}
