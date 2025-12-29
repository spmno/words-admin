use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    semester: String,
    unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sentence: Option<String>,
    meaning: String,
}

impl Word {
    pub fn new(input: &str) -> Self {
        let word: Word = serde_json::from_str(input).unwrap();
        Self {
            semester: word.semester,
            unit: word.unit,
            word: word.word,
            phrase: word.phrase,
            sentence: word.sentence,
            meaning: word.meaning,
        }
    }
}
