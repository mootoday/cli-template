use crate::cli_context::CliContext;
use crate::structs::CommandReturn;
use anyhow::Error;
use clap::Args;

/// Print the CLI version
#[derive(Debug, Args, Default)]
pub struct Command {}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<CommandReturn, Error> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(CommandReturn::default())
}
