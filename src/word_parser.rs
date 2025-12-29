use entity::word::ActiveModel;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WordParser {}

impl WordParser {
    pub fn new() -> Self {
        WordParser {}
    }

    pub fn parse(&self, file_name: &str) -> Result<Vec<ActiveModel>, String> {
        let file = File::open(file_name).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        let mut words = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let json_word: serde_json::Value =
                serde_json::from_str(&line).map_err(|e| format!("Failed to parse JSON: {}", e))?;

            let word = ActiveModel {
                id: sea_orm::ActiveValue::NotSet,
                semester: sea_orm::ActiveValue::Set(
                    json_word["semester"].as_str().unwrap_or("").to_string(),
                ),
                unit: sea_orm::ActiveValue::Set(
                    json_word["unit"].as_str().unwrap_or("").to_string(),
                ),
                word: sea_orm::ActiveValue::Set(json_word["word"].as_str().map(|s| s.to_string())),
                phrase: sea_orm::ActiveValue::Set(
                    json_word["phrase"].as_str().map(|s| s.to_string()),
                ),
                sentence: sea_orm::ActiveValue::Set(
                    json_word["sentence"].as_str().map(|s| s.to_string()),
                ),
                meaning: sea_orm::ActiveValue::Set(
                    json_word["meaning"].as_str().unwrap_or("").to_string(),
                ),
            };

            words.push(word);
        }

        Ok(words)
    }
}
