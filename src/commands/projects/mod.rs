use crate::{cli_context::CliContext, structs::CommandReturn};
use clap::{Args, Subcommand};
use clap_nested_commands::generate_async_commands;

mod create;
mod tasks;

/// Commands related to projects
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_async_commands!(return_type = CommandReturn;
    create,
    tasks
);
