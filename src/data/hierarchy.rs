use db::ArcDb;
use std::path::PathBuf;

pub struct App {
    pub db: ArcDb,
    pub app_home_path: PathBuf,
    pub hint: String,
}
impl App {
    pub async fn init(home_path: PathBuf) -> anyhow::Result<App> {
        let db = ArcDb::init_db(home_path.join("db")).await?;
        let commit = env!("GIT_COMMIT", "error");
        let branch = env!("GIT_BRANCH", "error");
        let build_date_time = env!("BUILD_DATE_TIME", "error");
        let hint = format!(
            r#"1. Current Git build version: {}-{}, build time: {}."#,
            branch, commit, build_date_time
        );
        Ok(App {
            db,
            app_home_path: home_path,
            hint,
        })
    }
}
