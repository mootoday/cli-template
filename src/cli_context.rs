use crate::CliArgs;

pub struct CliContext {}

impl From<&CliArgs> for CliContext {
    fn from(_args: &CliArgs) -> Self {
        Self {}
    }
}
