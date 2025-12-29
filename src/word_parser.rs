use crate::word::Word;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WordParser {}

impl WordParser {
    pub fn new() -> Self {
        WordParser {}
    }
    pub fn parse(&self, file_name: &str) -> Result<Vec<Word>, String> {
        let file = File::open(file_name).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        let mut words = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let word = Word::new(&line);
            words.push(word);
        }
        Ok(words)
    }
}
