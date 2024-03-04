use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub mod common;
pub mod hierarchy;
pub mod db;

pub struct WordDb {
    pub word_id: i64,
    pub word: String,
    pub zpk_path: String,
}

impl WordDb {
    pub async fn resource(&self, path: PathBuf) -> Result<WordResource> {
        let resource_data = tokio::fs::read(path.join("resource").join(self.zpk_path.as_str()).join("resource.json")).await?;
        let resource: WordResource = serde_json::from_slice(resource_data.as_slice())?;
        Ok(resource)
    }
}

/// 发音相似词？
#[derive(Serialize, Deserialize)]
pub struct Similar {
    #[serde(rename = "sId")]
    pub s_id: i64,
    pub word: String,
    #[serde(rename = "meanType")]
    pub mean_type: String,
    pub mean: String,
}
/// 反义词
/// {
///"aId": 10875,
///"word": "far"
///}
#[derive(Serialize, Deserialize)]
pub struct Antonym {
    #[serde(rename = "aId")]
    pub a_id: i64,
    pub word: String,
}

/// 同义词
/// {
///"sId": 349227,
///"word": "close"
///}
#[derive(Serialize, Deserialize)]
pub struct Synonym {
    #[serde(rename = "sId")]
    pub s_id: i64,
    pub word: String,
}

/// 变异：进行时、过去时
/// {
///"vId": 12225,
///"type": "现在分词",
///"variant": "nearing"
///}
#[derive(Serialize, Deserialize)]
pub struct Variant {
    #[serde(rename = "vId")]
    pub v_id: i64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub variant: String,
}

/// 短语
#[derive(Serialize, Deserialize)]
pub struct Phrase {
    #[serde(rename = "pId")]
    pub p_id: i64,
    pub phrase: String,
    pub mean: String,
}
/// 句子
#[derive(Serialize, Deserialize)]
pub struct Sentence {
    #[serde(rename = "sId")]
    pub s_id: i64,
    #[serde(rename = "sentenceEn")]
    pub sentence_en: String,
    pub translate: String,
    pub audio: String,
    pub origin: String,
    pub phrase: String,
}

/// 英文含义：{"mId":200812,"meanType":"prep.","mean":"at a short distance away from somebody/something"}
#[derive(Serialize, Deserialize)]
pub struct EnMean {
    #[serde(rename = "mId")]
    pub m_id: i64,
    #[serde(rename = "meanType")]
    pub mean_type: String,
    pub mean: String,
}

impl ToString for EnMean {
    fn to_string(&self) -> String {
        format!("{}\t{}", self.mean_type, self.mean)
    }
}

/// 中文含义
/// {
///"mId": 31576,
///"meanType": "prep.",
///"mean": "靠近",
///"percent": "75%"
///}
#[derive(Serialize, Deserialize)]
pub struct CnMean {
    #[serde(rename = "mId")]
    pub m_id: i64,
    #[serde(rename = "meanType")]
    pub mean_type: String,
    pub mean: String,
    pub percent: String,
}

impl ToString for CnMean {
    fn to_string(&self) -> String {
        format!("{}\t{}", self.mean_type, self.mean)
    }
}

/// 助记  "mnemonic": {
///"type": 3,
///"content": "你（n）的耳朵（ear）太“靠近”（near）我了。"
/// },
#[derive(Serialize, Deserialize)]
pub struct Mnemonic {
    #[serde(rename = "type")]
    pub r#type: i64,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Exam {
    pub recent: i64,
    #[serde(rename = "examName")]
    pub exam_name: String,
    pub nums: i64,
}

#[derive(Serialize, Deserialize)]
pub struct WordInfo {
    #[serde(rename = "topicId")]
    pub topic_id: i64,
    pub word: String,
    #[serde(rename = "wordSplit")]
    pub word_split: String,
    #[serde(rename = "accentUs")]
    pub accent_us: String,
    #[serde(rename = "accentUk")]
    pub accent_uk: String,
    #[serde(rename = "audioUs")]
    pub audio_us: String,
    #[serde(rename = "audioUk")]
    pub audio_uk: String,
    pub exam: Exam,
}

#[derive(Serialize, Deserialize)]
pub struct WordResource {
    pub word: WordInfo,
    pub mnemonic: Mnemonic,
    #[serde(rename = "cnMean")]
    pub cn_mean: Vec<CnMean>,
    #[serde(rename = "enMean")]
    pub en_mean: Vec<EnMean>,
    pub sentences: Vec<Sentence>,
    pub phrases: Vec<Phrase>,
    pub variant: Vec<Variant>,
    pub synonyms: Vec<Synonym>,
    pub antonyms: Vec<Antonym>,
    pub similars: Vec<Similar>,
}