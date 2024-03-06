use std::path::PathBuf;
use anyhow::anyhow;
use crate::resource::WordResource;

pub struct WordDb {
    pub word_id: i64,
    pub word: String,
    pub zpk_name: String,
}

impl WordDb {
    pub async fn resource(&self, app_home_path: PathBuf) -> anyhow::Result<WordResource> {
        let resource_path = self.zpk_path(app_home_path).join("resource.json");
        let resource_data = tokio::fs::read(resource_path.as_path()).await.map_err(|_| {
            anyhow!("read resource {:?} fail", resource_path)
        })?;
        let resource: WordResource = serde_json::from_slice(resource_data.as_slice())?;
        Ok(resource)
    }
    pub fn zpk_path(&self, app_home_path: PathBuf) -> PathBuf {
        app_home_path.join("resource").join(self.zpk_name.as_str())
    }
}