mod error;
pub mod view;

use crate::command::error::Error;
use crate::command::view::ViewConfig;
use crate::data::common::Config;
use crate::data::hierarchy::App;
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
    Ok(ViewConfig::init(&app, &Config::init(app.app_home_path.clone())))
}

#[command]
pub async fn review_info(state: State<'_, ArcApp>) -> Result<Vec<WordResourceView>> {
    let app = state.read().await;
    let now = chrono::Local::now().timestamp();
    let words = app.db.query_review_words(now, 40).await?;
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
