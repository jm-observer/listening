use serde::{Deserialize, Serialize};

/// 发音相似词？
#[derive(Serialize, Deserialize)]
pub struct Similar {
    #[serde(rename = "sId")]
    pub s_id: i64,
    pub word: String,
    #[serde(rename = "meanType", default)]
    pub mean_type: Option<String>,
    #[serde(default)]
    pub mean: Option<String>,
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
    pub phrase: String,
    pub image: Option<String>
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
///}
#[derive(Serialize, Deserialize)]
pub struct CnMean {
    #[serde(rename = "mId")]
    pub m_id: i64,
    #[serde(rename = "meanType")]
    pub mean_type: String,
    pub mean: String,
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
    #[serde(rename = "imgContent", default)]
    pub img_content: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
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
    #[serde(default)]
    pub exam: Option<Exam>,
}

#[derive(Serialize, Deserialize)]
pub struct WordResource {
    pub word: WordInfo,
    #[serde(default)]
    pub mnemonic: Option<Mnemonic>,
    #[serde(rename = "cnMean", default)]
    pub cn_mean: Vec<CnMean>,
    #[serde(rename = "enMean", default)]
    pub en_mean: Vec<EnMean>,
    #[serde(default)]
    pub sentences: Vec<Sentence>,
    #[serde(default)]
    pub phrases: Vec<Phrase>,
    #[serde(default)]
    pub variant: Vec<Variant>,
    #[serde(default)]
    pub synonyms: Vec<Synonym>,
    #[serde(default)]
    pub antonyms: Vec<Antonym>,
    #[serde(default)]
    pub similars: Vec<Similar>,
}


#[cfg(test)]
mod test {
    // use std::collections::HashMap;
    // use std::ffi::OsStr;
    // use std::path::PathBuf;
    use tokio::fs;
    use crate::resource::{WordResource};
    use crate::get_mime_type;

    #[test]
    fn test_mime_type() {
        assert_eq!(get_mime_type("abd.jpg").unwrap(), "image/jpeg".to_string());
        assert!(get_mime_type("fjpg").is_err());
    }

    #[tokio::test]
    async fn test_serde() {
        let mut dirs = tokio::fs::read_dir("D:\\u_unpack").await.unwrap();
        let mut pack_num= 0;
        // let mut tys = HashMap::new();
        // let mut path_extensions = HashMap::new();
        while let Some(pack) = dirs.next_entry().await.unwrap() {
            let Ok(data) = fs::read(pack.path().join("resource.json")).await else {
                println!("{:?} resource.json read fail", pack.file_name());
                continue;
            };
            match serde_json::from_slice::<WordResource>(data.as_ref()) {
                Err(e) => {
                    println!("{:?} \n{:?}", pack.file_name(), e);
                    break;
                }
                Ok(_rs) => {
                    // for x in _rs.sentences {
                    //     let Some(image) = &x.image else {
                    //         continue;
                    //     };
                    //     let image_path = pack.path().join(image);
                    //     let kind = infer::get_from_path(&image_path)
                    //         .expect("file read successfully")
                    //         .expect("file type is known");
                    //     let mime_type = kind.mime_type().to_string();
                    //     let extension = kind.extension().to_string();
                    //     let path_extension = image_path.extension().unwrap().to_str().unwrap();
                    //     path_extensions.insert(path_extension.to_string(), true);
                    //     if let Some(old_extension) = tys.insert(mime_type, extension.clone()) {
                    //         if !(old_extension == extension) {
                    //             println!("{} {}", old_extension, extension);
                    //         }
                    //     }
                    // }
                }
            }
            pack_num += 1;
        }
        // for (key, val) in tys {
        //     println!("{} {}", key, val);
        // }
        // for (key, _) in path_extensions {
        //     println!("{}", key);
        // }
        println!("pack_num: {}", pack_num);
    }


}


