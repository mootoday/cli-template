use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub show_feedback_prompt_for_errors: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            show_feedback_prompt_for_errors: true,
        }
    }
}

impl CliConfig {
    const APP_NAME: &'static str = "mootoday"; // TODO: Rename to match your company name
    const CONFIG_NAME: &'static str = "cli-template"; // TODO: Rename to match your CLI name

    pub fn load() -> Self {
        confy::load(Self::APP_NAME, Self::CONFIG_NAME).expect("Can't load CLI config file.")
    }

    pub fn disable_feedback_prompt_for_errors(&self) {
        self.update(|config| config.show_feedback_prompt_for_errors = false);
    }

    fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut config = self.clone();
        updater(&mut config);
        confy::store(Self::APP_NAME, Self::CONFIG_NAME, config)
            .expect("Can't store CLI config file.");
    }
}
