use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use tracing::{error, info};

use entity::word::{Column, Entity};
use sea_orm::{ColumnTrait, Database, DbConn, EntityTrait, QueryFilter};
use serde::Deserialize;
mod logging;

#[tokio::main]
async fn main() {
    // 初始化日志系统
    logging::init_logging();

    let db = Database::connect("sqlite://words.db")
        .await
        .expect("Failed to connect to database");
    let app = Router::new()
        .route("/", get(root))
        .route("/api/words", post(get_words_from_book))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello"
}

#[derive(Debug, Deserialize)]
pub struct WordQuery {
    semester: String,
    unit: String,
}

async fn get_words_from_book(
    State(db): State<DbConn>,
    Json(query): Json<WordQuery>,
) -> impl IntoResponse {
    let semester = &query.semester;
    let unit = &query.unit;

    match Entity::find()
        .filter(Column::Semester.eq(semester))
        .filter(Column::Unit.eq(unit))
        .all(&db)
        .await
    {
        Ok(words) => {
            info!(
                "Retrieved {} words for semester: {}, unit: {}",
                words.len(),
                semester,
                unit
            );
            (StatusCode::OK, Json(words))
        }
        Err(err) => {
            error!("Database error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        }
    }
}
