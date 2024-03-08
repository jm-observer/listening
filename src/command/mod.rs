mod error;
pub mod view;

use crate::command::error::Error;
use crate::command::view::ViewConfig;
use crate::data::common::Config;
use crate::data::hierarchy::App;
use log::{debug, warn};
use tauri::{command, State};
use tokio::sync::RwLock;

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
pub async fn review_info(state: State<'_, ArcApp>) -> Result<Vec<WordResourceView>> {
    let app = state.read().await;
    let now = chrono::Local::now().timestamp();
    let words = app.db.query_review_words(now, 5).await?;
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
pub async fn exam(rs: ExamRs, state: State<'_, ArcApp>) -> Result<()> {
    debug!("exam: {rs:?}");
    let now = chrono::Local::now().timestamp();
    let app = state.read().await;
    let rows_affected = match rs {
        ExamRs::Success { word_id } => {
            let record = app.db.query_learned_word(word_id).await?;
            let interval_hour = 8i64 * 2i64.pow(record.current_learned_times as u32);
            let next_time = interval_hour * 60 + now;
            app.db.exam_success(next_time, now, word_id).await?
        }
        ExamRs::Fail { word_id } => app.db.exam_fail(now, word_id).await?,
    };
    if rows_affected != 1 {
        warn!("update examine result fail");
    }
    Ok(())
}
