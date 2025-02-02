use crate::{cli_context::CliContext, structs::CommandReturn};
use anyhow::Error;
use clap::Args;

/// Add a new project task
#[derive(Debug, Args)]
pub struct Command {
    /// Name of the project task to add
    #[arg(long, short = 'n')]
    pub name: String,

    /// Description of the project task to add
    #[arg(long, short = 'd')]
    pub description: String,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<CommandReturn, Error> {
    Ok(CommandReturn {
        message: Some(format!(
            "Project task name: {}; description: {}",
            cmd.name, cmd.description
        )),
    })
}
