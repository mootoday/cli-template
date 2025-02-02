use std::process::exit;

use anyhow::Error;
use clap::{Parser, Subcommand};
use cli_context::CliContext;

mod cli_context;
mod commands;
mod structs;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
    // NOTE: If you add more CLI arguments here, adjust `./cli_context.rs` accordingly
}

#[derive(Debug, Subcommand)]
enum Commands {
    Projects(commands::projects::Command), // TODO: Replace this with your own CLI commands
    Version(commands::version::Command),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = CliArgs::parse();
    let cli_context = CliContext::from(&cli);

    let result = match cli.command {
        Commands::Projects(cmd) => commands::projects::execute(&cli_context, cmd).await,
        Commands::Version(cmd) => commands::version::execute(&cli_context, cmd).await,
    };

    match result {
        Ok(command_return) => {
            if command_return.message.is_some() {
                println!("{}\n\nThis is a dummy command, pleaes replace it with your own implementation.", command_return.message.unwrap_or_default());
            }
        }
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }

    Ok(())
}
