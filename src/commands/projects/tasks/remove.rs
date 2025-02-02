use crate::{cli_context::CliContext, structs::CommandReturn};
use anyhow::Error;
use clap::Args;

/// Remove a project task
#[derive(Debug, Args)]
pub struct Command {
    /// Name of the project task to remove
    #[arg(long, short = 'n')]
    pub name: String,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<CommandReturn, Error> {
    Ok(CommandReturn {
        message: Some(format!("Project task name: {}", cmd.name)),
    })
}
