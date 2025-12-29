use sea_orm::{ActiveModelTrait, Database, DbErr};
use words_admin::word_parser::WordParser;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite://words.db").await?;

    let book_names = make_book_names();

    for book_name in book_names {
        println!("parsing book:{}", book_name);
        let word_parser = WordParser::new();
        let words = word_parser
            .parse(&book_name)
            .expect("Failed to parse words");

        for word in words {
            word.insert(&db).await?;
        }

        println!("Successfully imported words from {}", book_name);
    }

    println!("All words imported successfully!");
    Ok(())
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
