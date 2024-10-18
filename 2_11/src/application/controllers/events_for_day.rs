use crate::application::event_params::EventParams;
use crate::application::AppState;
use crate::domain::TimeRange;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::ops::Deref;
use std::sync::Arc;

pub async fn events_for_day(
    State(state): State<Arc<AppState>>,
    Query(EventParams { user_id, date }): Query<EventParams>,
) -> (StatusCode, Json<Value>) {
    match state
        .event_service
        .events_for(TimeRange::DAY, user_id, date, state.repository.deref())
        .await
    {
        Ok(events) => (StatusCode::OK, Json(serde_json::json!(events))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": err.to_string()})),
        ),
    }
}
