use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use log::error;
use serde::{Deserialize, Serialize};
use anyhow::Result;
static U32: AtomicU32 = AtomicU32::new(0);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Id(u32);
impl Default for Id {
    fn default() -> Self {
        Self(U32.fetch_add(1, Ordering::Release))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub debug: bool,
}


impl Config {
    pub fn init(home_path: PathBuf) -> Self {
        let file_path = home_path.join("config.json");
        if let Ok(config) = Self::_init(file_path.clone()) {
            config
        } else {
            let config = Self::default();
            if let Err(e) = config.clone()._update(file_path) {
                error!("config update fail: {:?}", e);
            }
            config
        }
    }

    fn _init(file_path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(content.as_str())?;
        Ok(config)
    }

    fn _update(self, file_path: PathBuf) -> Result<()> {
        std::fs::write(file_path, serde_json::to_string(&self)?)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { debug: false }
    }
}
