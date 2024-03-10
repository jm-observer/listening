mod error;
pub mod view;

use crate::command::error::Error;
use crate::command::view::ViewConfig;
use crate::data::common::Config;
use crate::data::hierarchy::App;
use log::warn;
use tauri::{command, State};
use tokio::sync::RwLock;

use crate::data::db::*;
use crate::util::{during_today, during_yesterday};
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
pub async fn review_info(ty: ReviewTy, state: State<'_, ArcApp>) -> Result<Vec<WordResourceView>> {
    let app = state.read().await;
    let mut connect = app.db.get_connect().await?;
    let now = chrono::Local::now().timestamp();
    let words = match ty {
        ReviewTy::Today => {
            let (start, end) = during_today();
            query_during_right_words(connect.as_mut(), start, end).await?
        }
        ReviewTy::Yesterday => {
            let (start, end) = during_yesterday();
            query_during_right_words(connect.as_mut(), start, end).await?
        }
        ReviewTy::TodayError => {
            let (start, end) = during_today();
            query_during_error_words(connect.as_mut(), start, end).await?
        }
        ReviewTy::YesterdayError => {
            let (start, end) = during_yesterday();
            query_during_error_words(connect.as_mut(), start, end).await?
        }
        ReviewTy::Review => {
            query_review_words(app.db.get_connect().await?.as_mut(), now, 30).await?
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
    let now = chrono::Local::now().timestamp();
    let app = state.read().await;
    let mut tran = app.db.get_transaction().await?;
    let mut rs_num = 0;
    let rows_affected = match rs {
        ExamRs::Success => {
            let record = query_learned_word(tran.as_mut(), word_id).await?;
            let interval_hour = 8i64 * 2i64.pow(record.current_learned_times as u32);
            let next_time = interval_hour * 60 + now;
            rs_num = 1;
            exam_success(tran.as_mut(), next_time, now, word_id).await?
        }
        ExamRs::Fail => exam_fail(tran.as_mut(), now, word_id).await?,
    };
    add_test_record(tran.as_mut(), word_id, now, rs_num).await?;
    if rows_affected != 1 {
        warn!("update examine result fail");
    }
    tran.commit().await?;
    Ok(())
}
