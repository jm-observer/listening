use anyhow::{anyhow, Result};

use crate::data::hierarchy::App;
use model::db::WordDb;
use log::warn;
use sqlx::SqlitePool;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub db: SqlitePool,
}

impl ArcDb {
    pub async fn init_db(db_path: PathBuf) -> Result<Self> {
        if !db_path.exists() {
            tokio::fs::File::create(db_path.as_path()).await?;
        }
        let db_path_str = db_path
            .to_str()
            .ok_or(anyhow!("db path ({:?}) to str fail", db_path.as_path()))?;
        let db_url = format!("sqlite:{}", db_path_str);
        // println!("db path: {}, db url: {}", db_path_str, db_url);
        let db = SqlitePool::connect(db_url.as_str()).await?;
        Ok(ArcDb { db })
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

    pub async fn query_review_words(&self, next_time: i64, limit: i32) -> Result<Vec<WordDb>> {
        let rs = sqlx::query!(
            r#"
        SELECT word_id as "word_id!",word as "word!", zpk_name  from words w WHERE w.word_id in (
            SELECT word_id  from learned_word lw WHERE next_time < ? ORDER by next_time  limit ?)
        "#,
            next_time,
            limit
        )
        .fetch_all(self.db.acquire().await?.as_mut())
        .await?;
        let mut records = Vec::with_capacity(limit as usize);
        for word in rs {
            let Some(zpk_name) = word
                .zpk_name
                .map(|x| x.replace("\\/r\\/", "").replace("\\.zpk", ""))
            else {
                warn!("{}({}) zpk_path is none", word.word, word.word_id);
                continue;
            };
            let word = WordDb {
                word_id: word.word_id,
                word: word.word,
                zpk_name: zpk_name,
            };
            records.push(word);
        }
        Ok(records)
    }
}
