use crate::{cli_context::CliContext, structs::CommandReturn};
use clap::{Args, Subcommand};
use clap_nested_commands::generate_async_commands;

mod changelog;
mod print;
mod update;

/// Commands related to the CLI version
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

generate_async_commands!(
    return_type = CommandReturn,
    default_command = print;
    changelog,
    update
);
