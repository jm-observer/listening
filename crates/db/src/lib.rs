use anyhow::{anyhow, Result};
use log::warn;
use model::db::{LearnedWordDb, WordDb};
use sqlx::pool::PoolConnection;
use sqlx::{Sqlite, SqliteConnection, SqlitePool, Transaction};
use std::fs::File;
use std::path::PathBuf;
#[derive(Clone, Debug)]
pub struct ArcDb {
    pub db: SqlitePool,
}

impl ArcDb {
    pub async fn init_db(db_path: PathBuf) -> Result<Self> {
        if !db_path.exists() {
            File::create(db_path.as_path())?;
        }
        let db_path_str = db_path
            .to_str()
            .ok_or(anyhow!("db path ({:?}) to str fail", db_path.as_path()))?;
        let db_url = format!("sqlite:{}", db_path_str);
        // println!("db path: {}, db url: {}", db_path_str, db_url);
        let db = SqlitePool::connect(db_url.as_str()).await?;
        Ok(ArcDb { db })
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
    next_time: &String,
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
        };
        records.push(word);
    }
    Ok(records)
}

pub async fn query_during_error_words(
    connect: &mut SqliteConnection,
    start_time: &String,
    end_time: &String,
) -> Result<Vec<WordDb>> {
    log::debug!("{}-{}", start_time, end_time);
    let rs =
        sqlx::query!(
            r#"
            SELECT w.word_id as "word_id!",word as "word!", zpk_name from words w
                WHERE w.word_id  in (SELECT word_id FROM test_record tr where tr.result = 0 and tr.time > ? and tr.time < ?)
            "#,
            start_time,
            end_time
        )
            .fetch_all(connect)
            .await?
        ;
    let mut records = Vec::with_capacity(rs.len());
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
        };
        records.push(word);
    }
    Ok(records)
}

pub async fn query_during_right_words(
    connect: &mut SqliteConnection,
    start_time: &String,
    end_time: &String,
) -> Result<Vec<WordDb>> {
    log::debug!("{}-{}", start_time, end_time);
    let rs =
        sqlx::query!(
            r#"
            SELECT w.word_id as "word_id!",word as "word!", zpk_name from words w
                WHERE w.word_id  in (
                    SELECT word_id  from test_record tr2 where tr2."result" = 1 and tr2.time > ? and tr2.time < ?
                    EXCEPT
                    SELECT word_id  from test_record tr1 where tr1."result" = 0 and tr1.time > ? and tr1.time < ?
                )
            "#,
            start_time,
            end_time,
            start_time,
            end_time
        )
            .fetch_all(connect)
            .await?
        ;
    let mut records = Vec::with_capacity(rs.len());
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
    next_time: &String,
    last_time: &String,
    word_id: i64,
) -> Result<u64> {
    // and last_time < ?    避免短时间内重复提交
    let rows_affected = sqlx::query!(
        r#"
        update learned_word set last_time = ?
            , total_learned_times = total_learned_times + 1
            , current_learned_times = current_learned_times + 1
            , next_time = ? where learned_word.word_id = ? and last_time < ?
        "#,
        last_time,
        next_time,
        word_id,
        last_time
    )
    .execute(connect)
    .await?
    .rows_affected();
    Ok(rows_affected)
}

pub async fn exam_fail(
    connect: &mut SqliteConnection,
    last_time: &String,
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
    time: &String,
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

pub async fn add_audio_replace_record(
    connect: &mut SqliteConnection,
    word_id: i64,
    time: &str,
    word: &str,
) -> Result<i64> {
    let id = sqlx::query!(
        r#"
            insert into audio_replace_record(word_id, time, word) values(?, ?, ?)
        "#,
        word_id,
        time,
        word
    )
    .execute(connect)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn query_amount_of_waitint_to_review(
    connect: &mut SqliteConnection,
    now: &String,
) -> Result<i32> {
    let rs = sqlx::query!(
        r#"
        SELECT COUNT(*) as count from learned_word lw where lw.next_time <= ?
        "#,
        now,
    )
    .fetch_one(connect)
    .await?;
    Ok(rs.count)
}

pub async fn query_amount_of_tested(connect: &mut SqliteConnection, now: &String) -> Result<i32> {
    let rs = sqlx::query!(
        r#"
        SELECT COUNT(*) as count from learned_word lw where lw.next_time > ?
        "#,
        now,
    )
    .fetch_one(connect)
    .await?;
    Ok(rs.count)
}

pub async fn query_amount_of_today_tested(
    connect: &mut SqliteConnection,
    zero: &String,
) -> Result<i32> {
    let rs = sqlx::query!(
        r#"
        SELECT COUNT( DISTINCT word_id) as count from test_record tr where tr.time >= ?
        "#,
        zero,
    )
    .fetch_one(connect)
    .await?;
    Ok(rs.count)
}

pub async fn query_amount_of_today_tested_error(
    connect: &mut SqliteConnection,
    zero: &String,
) -> Result<i32> {
    let rs = sqlx::query!(
        r#"
        SELECT COUNT( DISTINCT word_id) as count  from test_record tr where tr.time >= ? and result = 0
        "#,
        zero,
    )
        .fetch_one(connect)
        .await?;
    Ok(rs.count)
}

pub async fn query_word(connect: &mut SqliteConnection, word: &str) -> Result<WordDb> {
    let word = sqlx::query!(
        r#"
        SELECT word_id,word, zpk_name
            from words w where w.word  = ?
        "#,
        word
    )
    .fetch_one(connect)
    .await?;
    let zpk_name = word
        .zpk_name
        .map(|x| x.replace("\\/r\\/", "").replace("\\.zpk", ""))
        .unwrap_or_default();
    let word = WordDb {
        word_id: word.word_id,
        word: word.word,
        zpk_name,
    };
    Ok(word)
}
