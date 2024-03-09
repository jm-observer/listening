use anyhow::{anyhow, Result};

use crate::data::hierarchy::App;
use log::warn;
use model::db::{LearnedWordDb, WordDb};
use sqlx::pool::PoolConnection;
use sqlx::{Sqlite, SqliteConnection, SqlitePool, Transaction};
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
            app_home_path: home_path,
            hint,
        })
    }

    pub async fn get_connect(&self) -> Result<PoolConnection<Sqlite>> {
        Ok(self.db.acquire().await?)
    }
    pub async fn get_transaction(&self) -> Result<Transaction<Sqlite>> {
        Ok(self.db.begin().await?)
    }
}

pub async fn query_review_words(
    connect: &mut SqliteConnection,
    next_time: i64,
    limit: i32,
) -> Result<Vec<WordDb>> {
    let rs = sqlx::query!(
            r#"
        SELECT w.word_id as "word_id!",word as "word!", zpk_name, lw.current_learned_times as "current_learned_times!"
            from words w, learned_word lw where w.word_id  = lw.word_id
                and lw.next_time < ? ORDER by next_time LIMIT ?
        "#,
            next_time,
            limit
        )
        .fetch_all(connect)
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
            zpk_name,
            current_learned_times: word.current_learned_times,
        };
        records.push(word);
    }
    Ok(records)
}

pub async fn query_learned_word(
    connect: &mut SqliteConnection,
    word_id: i64,
) -> Result<LearnedWordDb> {
    let rs = sqlx::query!(
        r#"
        SELECT current_learned_times as "current_learned_times!"
            , id as "id!"
            , word_id as "word_id!"
            , start_time as "start_time!"
            , last_time as "last_time!"
            , next_time as "next_time!"
            , err_times as "err_times!"
            , total_learned_times as "total_learned_times!"
            from learned_word where word_id  = ?
        "#,
        word_id
    )
    .fetch_one(connect)
    .await?;
    Ok(LearnedWordDb {
        id: rs.id,
        word_id,
        start_time: rs.start_time,
        last_time: rs.last_time,
        next_time: rs.next_time,
        err_times: rs.err_times,
        total_learned_times: rs.total_learned_times,
        current_learned_times: rs.current_learned_times,
    })
}

pub async fn exam_success(
    connect: &mut SqliteConnection,
    next_time: i64,
    last_time: i64,
    word_id: i64,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
        update learned_word set last_time = ?
            , total_learned_times = total_learned_times + 1
            , current_learned_times = current_learned_times + 1
            , next_time = ? where learned_word.word_id = ?
        "#,
        last_time,
        next_time,
        word_id
    )
    .execute(connect)
    .await?
    .rows_affected();
    Ok(rows_affected)
}

pub async fn exam_fail(
    connect: &mut SqliteConnection,
    last_time: i64,
    word_id: i64,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
            update learned_word set err_times = err_times + 1
                , last_time = ?
                , total_learned_times = total_learned_times + 1
                , current_learned_times = 0
                ,  next_time  = ? where learned_word.word_id = ?
        "#,
        last_time,
        last_time,
        word_id
    )
    .execute(connect)
    .await?
    .rows_affected();
    Ok(rows_affected)
}

pub async fn add_test_record(
    connect: &mut SqliteConnection,
    word_id: i64,
    time: i64,
    rs: i64,
) -> Result<i64> {
    let id = sqlx::query!(
        r#"
            insert into test_record(word_id, time, result) values(?, ?, ?)
        "#,
        word_id,
        time,
        rs
    )
    .execute(connect)
    .await?
    .last_insert_rowid();
    Ok(id)
}
