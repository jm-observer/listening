mod error;
pub mod view;

use crate::command::error::Error;
use crate::command::view::ViewConfig;
use crate::data::common::Config;
use crate::data::hierarchy::App;
use log::{debug, warn};
use std::time::Duration;
use tauri::{command, State};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::util::{date_time_str, during_today, during_yesterday, now_str, today_zero};
use db::*;
use model::view::*;

type ArcApp = RwLock<App>;
type Result<T> = std::result::Result<T, Error>;

// #[command]
// pub async fn init(datas: SubscribeInput, state: State<'_, ArcApp>) -> Result<()> {
//     debug!("subscribe: {:?}", datas);
//     let mut app = state.write().await;
//     Ok(())
// }

#[command]
pub async fn loading(state: State<'_, ArcApp>) -> Result<ViewConfig> {
    let app = state.read().await;
    Ok(ViewConfig::init(
        &app,
        &Config::init(app.app_home_path.clone()),
    ))
}

#[command]
pub async fn review_info(
    ty: ReviewTy,
    limit: i32,
    state: State<'_, ArcApp>,
) -> Result<Vec<WordResourceView>> {
    let app = state.read().await;
    let mut connect = app.db.get_connect().await?;
    let words = match ty {
        ReviewTy::Today => {
            let (start, end) = during_today();
            query_during_right_words(connect.as_mut(), &start, &end).await?
        }
        ReviewTy::Yesterday => {
            let (start, end) = during_yesterday();
            query_during_right_words(connect.as_mut(), &start, &end).await?
        }
        ReviewTy::TodayError => {
            let (start, end) = during_today();
            query_during_error_words(connect.as_mut(), &start, &end).await?
        }
        ReviewTy::YesterdayError => {
            let (start, end) = during_yesterday();
            query_during_error_words(connect.as_mut(), &start, &end).await?
        }
        ReviewTy::Review => {
            query_review_words(
                app.db.get_connect().await?.as_mut(),
                &date_time_str(chrono::Local::now()),
                limit,
            )
            .await?
        }
    };
    let mut view_tasks = Vec::with_capacity(words.len());
    for word in words {
        let home_path = app.app_home_path.clone();
        view_tasks.push(tokio::spawn(WordResourceView::init(word, home_path)));
    }
    let mut rs = Vec::with_capacity(view_tasks.len());
    for view in view_tasks {
        match view.await? {
            Ok(view) => {
                rs.push(view);
            }
            Err(err) => {
                log::warn!("{}", err.to_string());
            }
        }
    }
    Ok(rs)
}
#[command]
pub async fn exam(rs: ExamRs, word_id: i64, state: State<'_, ArcApp>) -> Result<()> {
    let now = chrono::Local::now();
    let app = state.read().await;
    let mut tran = app.db.get_transaction().await?;
    let mut rs_num = 0;
    let rows_affected = match rs {
        ExamRs::Success => {
            let record = query_learned_word(tran.as_mut(), word_id).await?;
            let interval_hour = 12u64 * 2u64.pow(record.current_learned_times as u32);
            let next_time = now + Duration::from_secs(interval_hour * 60 * 60);
            rs_num = 1;
            exam_success(
                tran.as_mut(),
                &date_time_str(next_time),
                &date_time_str(now),
                word_id,
            )
            .await?
        }
        ExamRs::Fail => exam_fail(tran.as_mut(), &date_time_str(now), word_id).await?,
    };
    add_test_record(tran.as_mut(), word_id, &date_time_str(now), rs_num).await?;
    if rows_affected != 1 {
        warn!("update examine result fail");
    }
    tran.commit().await?;
    Ok(())
}
#[command]
pub async fn replace_audio(
    word_id: i64,
    word: String,
    audio_path: String,
    state: State<'_, ArcApp>,
) -> Result<()> {
    debug!("{} {} {}", word_id, word, audio_path);
    let app = state.read().await;
    let mut tran = app.db.get_transaction().await?;
    let word_audio_url = format!("https://dict.youdao.com/dictvoice?type=2&audio={}", word);
    let resp = reqwest::get(word_audio_url.as_str()).await?.bytes().await?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(audio_path)
        .await?;
    file.write_all(resp.as_ref()).await?;
    file.flush().await?;
    add_audio_replace_record(tran.as_mut(), word_id, &now_str(), &word).await?;
    tran.commit().await?;
    Ok(())
}
#[command]
pub async fn load_overview(state: State<'_, ArcApp>) -> Result<Overview> {
    let app = state.read().await;
    let mut tran = app.db.get_transaction().await?;
    let today_zero_time = date_time_str(today_zero());

    let tested_amount =
        query_amount_of_tested(tran.as_mut(), &date_time_str(chrono::Local::now())).await?;
    let waiting_amount =
        query_amount_of_waitint_to_review(tran.as_mut(), &date_time_str(chrono::Local::now()))
            .await?;
    let today_all_amount = query_amount_of_today_tested(tran.as_mut(), &today_zero_time).await?;
    let today_error_amount =
        query_amount_of_today_tested_error(tran.as_mut(), &today_zero_time).await?;
    Ok(Overview {
        tested_amount,
        waiting_amount,
        today_all_amount,
        today_error_amount,
    })
}
