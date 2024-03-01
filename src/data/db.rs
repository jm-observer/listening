use anyhow::Result;

use sled::{Config, Db};
use std::path::PathBuf;
use crate::data::hierarchy::App;

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub index: usize,
    pub db: Db,
    pub ids: Vec<usize>,
}

impl ArcDb {
    pub fn init_db(db_path: PathBuf) -> Result<Self> {
        let config = Config::new().path(db_path);
        Ok(ArcDb {
            index: 0,
            db: config.open()?,
            ids: Default::default(),
        })
    }

    pub fn next_broker_id(&mut self) -> usize {
        self.index += 1;
        self.index
    }


    pub fn read_app_data(&mut self, home_path: PathBuf) -> Result<App> {
        let commit = env!("GIT_COMMIT", "error");
        let branch = env!("GIT_BRANCH", "error");
        let build_date_time = env!("BUILD_DATE_TIME", "error");
        let hint = format!(
            r#"1. Current Git build version: {}-{}, build time: {}."#,
            branch, commit, build_date_time
        );
        Ok(App {
            db: self.clone(),
            home_path,
            hint,
        })
    }
}
