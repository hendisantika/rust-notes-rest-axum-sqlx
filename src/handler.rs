use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{NoteModel, NoteModelResponse},
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    AppState,
};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // // Query with macro
    // let notes = sqlx::query_as!(
    //     NoteModel,
    //     r#"SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?"#,
    //     limit as i32,
    //     offset as i32
    // )
    // .fetch_all(&data.db)
    // .await
    // .map_err(|e| {
    //     let error_response = serde_json::json!({
    //         "status": "error",
    //         "message": format!("Database error: { }", e),
    //     });
    //     (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    // })?;

    // Query without macro
    let notes =
        sqlx::query_as::<_, NoteModel>(r#"SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?"#)
            .bind(limit as i32)
            .bind(offset as i32)
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Database error: { }", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    // Response
    let note_responses = notes
        .iter()
        .map(|note| to_note_response(&note))
        .collect::<Vec<NoteModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": note_responses.len(),
        "notes": note_responses
    });

    Ok(Json(json_response))
}