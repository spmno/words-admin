use words_admin::word_parser::WordParser;

fn main() {
    let book_names = make_book_names();
    for book_name in book_names {
        println!("parsing book:{}", book_name);
        let word_parser = WordParser::new();
        let words = word_parser.parse(&book_name).unwrap();
        println!("Parsed words from {:?}", words);
        // Insert words into database
    }
}

fn make_book_names() -> Vec<String> {
    vec![
        "textbook/jsonl/3_first.json".to_string(),
        "textbook/jsonl/4_first.json".to_string(),
        "textbook/jsonl/5_first.json".to_string(),
        "textbook/jsonl/6_first.json".to_string(),
        "textbook/jsonl/7_first.json".to_string(),
        "textbook/jsonl/8_first.json".to_string(),
    ]
}
