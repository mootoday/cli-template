use crate::cli_context::CliContext;
use crate::structs::CommandReturn;
use anyhow::Error;
use clap::Args;
use tokio::task;

/// Update the CLI
#[derive(Debug, Args)]
pub struct Command {}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<CommandReturn, Error> {
    let status = task::spawn_blocking(|| {
        self_update::backends::github::Update::configure()
            .repo_owner("mootoday")
            .repo_name("cli-template")
            .bin_name(env!("CARGO_BIN_NAME"))
            .show_download_progress(true)
            .current_version(env!("CARGO_PKG_VERSION"))
            .build()
            .and_then(|update| update.update())
    })
    .await??;
    let status_message = match status {
        self_update::Status::UpToDate(_) => String::from("You already use the latest version."),
        self_update::Status::Updated(version) => format!("CLI updated to {version}"),
    };
    println!("\n{status_message}");
    Ok(CommandReturn::default())
}
