use crate::{cli_config::CliConfig, CliArgs};

pub struct CliContext {
    pub config: CliConfig,
}

impl From<&CliArgs> for CliContext {
    fn from(_args: &CliArgs) -> Self {
        Self {
            config: CliConfig::load(),
        }
    }
}
