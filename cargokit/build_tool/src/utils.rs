use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result};
use log::debug;

pub fn run_command(mut command: Command) -> Result<String> {
    debug!("Running command ${:?}", command);

    let output = command
        .output()
        .with_context(|| format!("Failed to run command: {:?}", command.get_program()))?;

    #[allow(unused_mut)]
    let mut success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !success {
        Err(anyhow::format_err!(
            "Command {:?} failed with error {};\nstderr: {}\nstdout: {}",
            command,
            output.status,
            String::from_utf8(output.stderr).unwrap(),
            stdout
        ))
    } else {
        Ok(stdout.into())
    }
}

pub fn installed_targets() -> Result<Vec<String>> {
    let mut cmd = Command::new("rustup");
    cmd.args(["target", "list", "--installed"]);
    let installed = run_command(cmd)?;
    Ok(installed.split('\n').map(String::from).collect())
}

pub fn install_target(target: &str) -> Result<()> {
    let mut cmd = Command::new("rustup");
    cmd.args(["target", "add", target]);
    run_command(cmd)?;
    Ok(())
}

pub fn string_from_env(var: &str) -> Result<String> {
    std::env::var(var).with_context(|| format!("Missing environment variable: {}", var))
}

pub fn path_from_env(var: &str) -> Result<PathBuf> {
    Ok(string_from_env(var)?.into())
}
