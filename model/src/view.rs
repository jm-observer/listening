use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::db::WordDb;
use crate::resource::{Sentence, WordInfo, WordResource};
use crate::resource_path;

#[derive(Serialize, Deserialize)]
pub struct WordResourceView {
    pub word: WordInfoView,
    pub cn_mean: Vec<String>,
    pub en_mean: Vec<String>,
    pub sentences: Vec<SentenceView>,
    pub image: Option<String>
}

impl WordResourceView {
    pub async fn init(word_db: WordDb, home_path: PathBuf) -> anyhow::Result<Self> {
        let resource = word_db.resource(home_path).await?;
        let WordResource {
            word,
            cn_mean,
            en_mean,
            sentences,
            ..
        } = resource;
        let word = WordInfoView::init(word, resource_path(), word_db.zpk_name.as_str());
        let sentences: Vec<SentenceView> = sentences
            .into_iter()
            .map(|x| SentenceView::init(x, resource_path(), word_db.zpk_name.as_str()))
            .collect();
        let cn_mean: Vec<String> = cn_mean.iter().map(|x| x.to_string()).collect();
        let en_mean: Vec<String> = en_mean.iter().map(|x| x.to_string()).collect();
        let image = sentences.iter().find_map(|x| x.image.clone());
        Ok(Self {
            word,
            cn_mean,
            en_mean,
            sentences, image
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
    pub fn init(word_info: WordInfo, dir: &str, zpk_name: &str) -> Self {
        let audio_us = format!("{}\\{}\\{}", dir, zpk_name, word_info.audio_us);
        let audio_uk = format!("{}\\{}\\{}", dir, zpk_name, word_info.audio_uk);
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
    pub fn init(sentence: Sentence, dir: &str, zpk_name: &str) -> Self {
        let audio = format!("{}\\{}\\{}", dir, zpk_name, sentence.audio);
        let image: Option<String> = sentence.image.map(|x| format!("{}\\{}\\{}", dir, zpk_name, x));
        Self {
            sentence: sentence.sentence_en,
            translate: sentence.translate,
            audio,
            image
        }
    }
}
