use crate::{cli_context::CliContext, structs::CommandReturn};
use anyhow::Error;
use clap::Args;
use cliclack::{input, intro, note, outro};
use serde::Serialize;

const DISCORD_WEBHOOK_URL: &str = "https://discord.com/api/webhooks/1335747745826345012/1u0bzP6nN3KElySc3MVMUuYW2qWRRIfyE_RytxqxBtbr70zMat3q1OHBeVHwekLHZhs4"; // TODO: Replace with your own Discord, Slack, etc. integration

struct FeedbackPrompt {
    heading: &'static str,
    subheading: &'static str,
    placeholder: &'static str,
}

#[derive(Debug, Serialize)]
struct FeedbackPayload {
    content: String,
}

/// Provide feedback
#[derive(Debug, Args)]
pub struct Command {
    /// Determines if the feedback command is used as part of an error that occurred
    #[arg(long, hide = true)]
    pub is_error_feedback: bool,
}

pub async fn execute(_cli_context: &CliContext, cmd: Command) -> Result<CommandReturn, Error> {
    let prompt = if cmd.is_error_feedback {
        FeedbackPrompt {
            heading: "Oh no...",
            subheading: "We apologize for the inconvenience",
            placeholder: "Please let us know what happened",
        }
    } else {
        intro("Feedback")?;
        FeedbackPrompt {
            heading: "Thank you",
            subheading: "We appreciate your feedback",
            placeholder: "What works? What doesn't? What do you wish the CLI did?",
        }
    };

    note(prompt.heading, prompt.subheading)?;
    let feedback: String = input("")
        .required(false)
        .placeholder(prompt.placeholder)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please provide feedback or cancel with Ctrl + C")
            } else {
                Ok(())
            }
        })
        .interact()?;

    reqwest::Client::new()
        .post(DISCORD_WEBHOOK_URL)
        .json(&FeedbackPayload { content: feedback })
        .send()
        .await
        .ok();
    outro("Feedback submitted.")?;

    Ok(CommandReturn::default())
}
