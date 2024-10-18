use crate::application::event_params::EventParams;
use crate::application::AppState;
use crate::domain::{Event, TimeRange};
use axum::body::Body;
use axum::extract::{FromRequestParts, Query, State};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use serde::de::IntoDeserializer;
use serde_json::{json, Value};
use std::ops::Deref;
use std::sync::Arc;

pub async fn events_for_week(
    State(state): State<Arc<AppState>>,
    Query(EventParams { user_id, date }): Query<EventParams>,
) -> (StatusCode, Json<Value>) {
    match state
        .event_service
        .events_for(TimeRange::WEEK, user_id, date, state.repository.deref())
        .await
    {
        Ok(events) => (StatusCode::OK, Json(serde_json::json!(events))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": err.to_string()})),
        ),
    }
}
