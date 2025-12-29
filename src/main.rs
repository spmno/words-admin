use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};

use crate::word::{Column, Entity, Model};
use crate::word_parser::WordParser;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello"
}

async fn get_words_from_book(book_name: &str) -> Json<Vec<Model>> {
    let word_parser = WordParser::new();
    let active_models = word_parser.parse(book_name).unwrap();
    // Convert ActiveModel to Model would require database insertion
    // This is a placeholder - in practice you'd query the database
    Json(Vec::new())
}
