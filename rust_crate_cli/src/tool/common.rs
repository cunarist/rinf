use crate::SetupError;
use std::net::ToSocketAddrs;
use std::process::Command;

pub fn run_dart_command(args: &[&str]) -> Result<(), SetupError> {
    #[cfg(target_family = "windows")]
    let cmd = "dart.bat";
    #[cfg(target_family = "unix")]
    let cmd = "dart";
    run_subprocess(cmd, args)
}

pub fn run_subprocess(cmd: &str, args: &[&str]) -> Result<(), SetupError> {
    Command::new(cmd).args(args).status()?;
    Ok(())
}

pub fn check_internet_connection() -> bool {
    "pub.dev:80"
        .to_socket_addrs()
        .map(|mut addrs| addrs.next().is_some())
        .unwrap_or(false)
}
