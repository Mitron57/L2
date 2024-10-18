use crate::application::AppState;
use crate::domain::Event;
use axum::body::Body;
use axum::extract::{FromRequestParts, Query, State};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use serde::de::IntoDeserializer;
use serde_json::{json, Value};
use std::ops::Deref;
use std::sync::Arc;

pub async fn create_event(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let Event {
        user_id,
        date,
        content,
    } = serde_json::from_value(payload).unwrap();
    match state
        .event_service
        .create_event(user_id, date, content, state.repository.deref())
        .await
    {
        Ok(_) => (StatusCode::CREATED, Json(json!({}))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": err.to_string()})),
        ),
    }
}
