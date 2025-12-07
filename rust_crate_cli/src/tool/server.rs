use crate::SetupError;
use arboard::Clipboard;

pub fn provide_server_command(release: bool) -> Result<(), SetupError> {
  let mut clipboard = Clipboard::new()?;
  let release_arg = if release { " --release" } else { "" };
  let base_command = concat!(
    "flutter run",
    " --web-header=cross-origin-opener-policy=same-origin",
    " --web-header=cross-origin-embedder-policy=require-corp",
  );
  let full_command = format!("{base_command}{release_arg}");
  clipboard.set_text(full_command)?;
  Ok(())
}
