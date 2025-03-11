use crate::SetupError;
use arboard::Clipboard;
use owo_colors::OwoColorize;

pub fn provide_server_command() -> Result<(), SetupError> {
    let mut clipboard = Clipboard::new()?;
    let full_command = concat!(
        "flutter run",
        " --web-header=Cross-Origin-Opener-Policy=same-origin",
        " --web-header=Cross-Origin-Embedder-Policy=require-corp",
    );
    clipboard.set_text(full_command)?;
    let full_guide = concat!(
        "Full `flutter run` command for the web",
        " has been copied to the clipboard"
    );
    println!("{}", full_guide.dimmed());
    Ok(())
}
