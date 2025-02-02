use crate::{cli_context::CliContext, structs::CommandReturn};
use clap::{Args, Subcommand};
use clap_nested_commands::generate_async_commands;

mod add;
mod remove;

/// Commands related to project tasks
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_async_commands!(return_type = CommandReturn;
    add,
    remove
);
