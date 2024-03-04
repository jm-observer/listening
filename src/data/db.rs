use anyhow::{anyhow, Result};

use std::path::PathBuf;
use sqlx::SqlitePool;
use crate::data::hierarchy::App;

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub db: SqlitePool,
}

impl ArcDb {
    pub async fn init_db(db_path: PathBuf) -> Result<Self> {
        if !db_path.exists() {
            tokio::fs::File::create(db_path.as_path()).await?;
        }
        let db_path_str = db_path.to_str().ok_or(anyhow!("db path ({:?}) to str fail", db_path.as_path()))?;
        let db_url = format!("sqlite:{}", db_path_str);
        // println!("db path: {}, db url: {}", db_path_str, db_url);
        let db = SqlitePool::connect(db_url.as_str()).await?;
        Ok(ArcDb {
            db
        })
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
