use crate::application::event_params::EventParams;
use crate::application::AppState;
use crate::domain::Event;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::ops::Deref;
use std::sync::Arc;

pub async fn delete_event(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let Event { user_id, date, .. } = serde_json::from_value(payload).unwrap();
    match state
        .event_service
        .delete_event(user_id, date, state.repository.deref())
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({}))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": err.to_string()})),
        ),
    }
}
