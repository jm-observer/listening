// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use data::db::ArcDb;
use custom_utils::logger::*;
use directories::UserDirs;
use log::LevelFilter::{Debug, Info};
use tokio::sync::RwLock;

mod data;
mod util;
mod command;

use command::*;
use util::APP;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user_dirs = UserDirs::new().unwrap();
    let home_path = user_dirs.home_dir().to_path_buf().join(APP);

    if !home_path.exists() {
        std::fs::create_dir_all(home_path.as_path())?;
    }
    let fs_path = home_path.clone();
    let fs = FileSpec::default()
        .directory(fs_path)
        .basename(APP)
        .suffix("log");
    // 若为true，则会覆盖rotate中的数字、keep^
    let criterion = Criterion::AgeOrSize(Age::Day, 10_000_000);
    let naming = Naming::Numbers;
    let cleanup = Cleanup::KeepLogFiles(2);
    let append = true;

    let _logger = custom_utils::logger::logger_feature_with_path(
        "listening",
        Debug,
        Info,
        home_path.clone(),
        home_path.clone(),
    )
    // .module("for_mqtt_client::protocol::packet", Info)
    .config(fs, criterion, naming, cleanup, append)
    // .log_to_write(Box::new(CustomWriter(tx.clone())))
    .build();

    // panic::set_hook(Box::new(|panic_info| {
    //     error!("{:?}", Backtrace::new());
    //     if let Some(location) = panic_info.location() {
    //         error!(
    //             "panic occurred in file '{}' at line {}",
    //             location.file(),
    //             location.line(),
    //         );
    //     }
    //     exit(1);
    // }));

    info!("home path: {:?}", home_path);
    let mut db = ArcDb::init_db(home_path.join("db")).await?;
    let data = db.read_app_data(home_path)?;

    tauri::Builder::default()
        .manage(RwLock::new(data))
        .invoke_handler(tauri::generate_handler![
            loading
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
