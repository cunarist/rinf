use crate::SetupError;
use arboard::Clipboard;

pub fn provide_server_command() -> Result<(), SetupError> {
  let mut clipboard = Clipboard::new()?;
  let full_command = concat!(
    "flutter run",
    " --web-header=Cross-Origin-Opener-Policy=same-origin",
    " --web-header=Cross-Origin-Embedder-Policy=require-corp",
  );
  clipboard.set_text(full_command)?;
  Ok(())
}
