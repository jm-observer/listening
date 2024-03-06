use crate::data::common::Config;
use crate::data::hierarchy::App;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewConfig {
    hint: String,
    debug: bool,
}

impl ViewConfig {
    pub fn init(app: &App, config: &Config) -> Self {
        Self {
            hint: app.hint.clone(),
            debug: config.debug,
        }
    }
}
