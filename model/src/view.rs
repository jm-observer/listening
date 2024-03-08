use crate::db::WordDb;
use crate::resource::{Sentence, WordInfo, WordResource};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct WordResourceView {
    pub word: WordInfoView,
    pub cn_mean: Vec<String>,
    pub en_mean: Vec<String>,
    pub sentences: Vec<SentenceView>,
    pub image: Option<String>,
    pub current_learned_times: i64,
}

impl WordResourceView {
    pub async fn init(word_db: WordDb, app_home_path: PathBuf) -> anyhow::Result<Self> {
        let zk_path = word_db.zpk_path(app_home_path.clone());
        let zk_path_str = zk_path
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("zk_path to string fail"))?;
        let resource = word_db.resource(app_home_path.clone()).await?;
        let WordResource {
            word,
            cn_mean,
            en_mean,
            sentences,
            ..
        } = resource;
        let word = WordInfoView::init(word, zk_path_str);
        let sentences: Vec<SentenceView> = sentences
            .into_iter()
            .map(|x| SentenceView::init(x, zk_path_str))
            .collect();
        let cn_mean: Vec<String> = cn_mean.iter().map(|x| x.to_string()).collect();
        let en_mean: Vec<String> = en_mean.iter().map(|x| x.to_string()).collect();
        let image = sentences.iter().find_map(|x| x.image.clone());
        Ok(Self {
            word,
            cn_mean,
            en_mean,
            sentences,
            image,
            current_learned_times: word_db.current_learned_times,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImageView {
    pub path: String,
    pub extend: String,
}

#[derive(Serialize, Deserialize)]
pub struct WordInfoView {
    pub word_id: i64,
    pub word: String,
    pub accent_us: String,
    pub accent_uk: String,
    pub audio_us: String,
    pub audio_uk: String,
}

impl WordInfoView {
    pub fn init(word_info: WordInfo, dir: &str) -> Self {
        let audio_us = format!("{}\\{}", dir, word_info.audio_us);
        let audio_uk = format!("{}\\{}", dir, word_info.audio_uk);
        Self {
            word_id: word_info.topic_id,
            word: word_info.word,
            accent_us: word_info.accent_us,
            accent_uk: word_info.accent_uk,
            audio_us,
            audio_uk,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SentenceView {
    pub sentence: String,
    pub translate: String,
    pub audio: String,
    pub image: Option<String>,
}

impl SentenceView {
    pub fn init(sentence: Sentence, dir: &str) -> Self {
        let audio = format!("{}\\{}", dir, sentence.audio);
        let image: Option<String> = sentence.image.map(|x| format!("{}\\{}", dir, x));
        Self {
            sentence: sentence.sentence_en,
            translate: sentence.translate,
            audio,
            image,
        }
    }
}
#[derive(Deserialize, Debug)]
#[serde(tag = "rs", rename_all = "snake_case")]
pub enum ExamRs {
    Success { word_id: i64 },
    Fail { word_id: i64 },
}
#[cfg(test)]
mod test {
    use crate::view::ExamRs;

    #[test]
    pub fn test_deser() {
        let val_str = r#"{"rs": "success", "word_id": 1}"#;
        let _: ExamRs = serde_json::from_str(val_str).unwrap();
        let val_str = r#"{"rs": "fail", "word_id": 1}"#;
        let _: ExamRs = serde_json::from_str(val_str).unwrap();
    }
}
