use crate::data::db::ArcDb;
use std::path::PathBuf;

pub struct App {
    pub db: ArcDb,
    pub home_path: PathBuf,
    pub hint: String,
}
impl App {
}
