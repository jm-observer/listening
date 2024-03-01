mod error;
pub mod view;

use tauri::{command, State};
use crate::command::error::Error;
use crate::data::hierarchy::App;
use tokio::sync::RwLock;
use crate::command::view::ViewConfig;
use crate::data::common::Config;

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
    let mut app = state.write().await;
    Ok(ViewConfig::init(&app, &Config::init(app.home_path.clone())))
}
