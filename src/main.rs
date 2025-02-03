use std::process::exit;

use anyhow::Error;
use clap::{Parser, Subcommand};
use cli_context::CliContext;
use cliclack::{log, outro};

mod cli_config;
mod cli_context;
mod commands;
mod structs;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ErrorFeedbackChoice {
    Yes,
    No,
    Never,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
    // NOTE: If you add more CLI arguments here, adjust `./cli_context.rs` accordingly
}

#[derive(Debug, Subcommand)]
enum Commands {
    Feedback(commands::feedback::Command),
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
        Commands::Feedback(cmd) => commands::feedback::execute(&cli_context, cmd).await,
    };

    match result {
        Ok(command_return) => {
            if command_return.message.is_some() {
                log::success(format!("{}", command_return.message.unwrap_or_default()))?;
                outro("This is a dummy command, pleaes replace it with your own implementation.")?;
            }
        }
        Err(err) => {
            log::error(format!("{}", err))?;

            if cli_context.config.show_feedback_prompt_for_errors {
                let selected = cliclack::select::<ErrorFeedbackChoice>(
                    "Would you like to provide feedback about this error?",
                )
                .item(ErrorFeedbackChoice::Yes, "Yes", "")
                .item(ErrorFeedbackChoice::No, "No", "")
                .item(
                    ErrorFeedbackChoice::Never,
                    "No, don't ask again",
                    "You can change this in the CLI's configuration file",
                )
                .interact()?;

                match selected {
                    ErrorFeedbackChoice::Yes => {
                        let feedback_cmd = commands::feedback::Command {
                            is_error_feedback: true,
                        };
                        _ = commands::feedback::execute(&cli_context, feedback_cmd).await;
                    }
                    ErrorFeedbackChoice::No => {}
                    ErrorFeedbackChoice::Never => {
                        cli_context.config.disable_feedback_prompt_for_errors();
                    }
                }
            }

            exit(1);
        }
    }

    Ok(())
}
