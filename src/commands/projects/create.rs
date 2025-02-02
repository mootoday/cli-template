use crate::{cli_context::CliContext, structs::CommandReturn};
use anyhow::Error;
use clap::Args;

/// Create a new project
#[derive(Debug, Args)]
pub struct Command {
    /// Name of the project to create
    #[arg(long, short = 'n')]
    pub name: String,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<CommandReturn, Error> {
    Ok(CommandReturn {
        message: Some(format!("Project name: {}", cmd.name)),
    })
}
