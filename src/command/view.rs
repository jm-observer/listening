use std::path::PathBuf;

use crate::data::common::Config;
use crate::data::hierarchy::App;
use crate::data::{Sentence, WordDb, WordInfo, WordResource};
use crate::util::resource_path;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewConfig {
    hint: String,
    debug: bool,
}

impl ViewConfig {
    pub fn init(app: &App, config: &Config) -> Self {
        Self {
            hint: app.hint.clone(),
            debug: config.debug,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WordResourceView {
    pub word: WordInfoView,
    pub cn_mean: Vec<String>,
    pub en_mean: Vec<String>,
    pub sentences: Vec<SentenceView>,
}

impl WordResourceView {
    pub async fn init(word_db: WordDb, home_path: PathBuf) -> Result<Self> {
        let resource = word_db.resource(home_path).await?;
        let WordResource {
            word,
            cn_mean,
            en_mean,
            sentences,
            ..
        } = resource;
        let word = WordInfoView::init(word, word_db.zpk_path.as_str());
        let sentences: Vec<SentenceView> = sentences
            .into_iter()
            .map(|x| SentenceView::init(x, word_db.zpk_path.as_str()))
            .collect();
        let cn_mean: Vec<String> = cn_mean.iter().map(|x| x.to_string()).collect();
        let en_mean: Vec<String> = en_mean.iter().map(|x| x.to_string()).collect();
        Ok(Self {
            word,
            cn_mean,
            en_mean,
            sentences,
        })
    }
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
    pub fn init(word_info: WordInfo, zk_name: &str) -> Self {
        let audio_us = format!("{}\\{}\\{}", resource_path(), zk_name, word_info.audio_us);
        let audio_uk = format!("{}\\{}\\{}", resource_path(), zk_name, word_info.audio_uk);
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
}

impl SentenceView {
    pub fn init(sentence: Sentence, zk_name: &str) -> Self {
        let audio = format!("{}\\{}\\{}", resource_path(), zk_name, sentence.audio);
        Self {
            sentence: sentence.sentence_en,
            translate: sentence.translate,
            audio,
        }
    }
}
