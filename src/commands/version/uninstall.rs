use std::{env, fs};

use crate::cli_context::CliContext;
use crate::structs::CommandReturn;
use anyhow::{bail, Error};
use clap::Args;
use cliclack::confirm;

/// Uninstall the CLI
#[derive(Debug, Args, Default)]
pub struct Command {}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<CommandReturn, Error> {
    let should_continue = confirm(format!(
        "Are you sure you want to uninstall the {} CLI?",
        env!("CARGO_BIN_NAME")
    ))
    .interact()?;

    if should_continue {
        uninstall()?;
        Ok(CommandReturn {
            message: Some(format!(
                "Successfully uninstalled the {} CLI",
                env!("CARGO_BIN_NAME")
            )),
        })
    } else {
        Ok(CommandReturn {
            message: Some(String::from("Nothing was uninstalled.")),
        })
    }
}

fn uninstall() -> Result<String, Error> {
    if let Ok(current_exe) = env::current_exe() {
        if let Err(e) = fs::remove_file(&current_exe) {
            bail!("Failed to uninstall: {}", e)
        } else {
            Ok(format!(
                "Successfully uninstalled the {} CLI.",
                env!("CARGO_BIN_NAME")
            ))
        }
    } else {
        bail!("Could not determine the CLI executable path")
    }
}
