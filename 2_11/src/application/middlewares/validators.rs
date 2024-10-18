use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::Json;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct CreateEventParams {
    user_id: usize,
    #[serde(deserialize_with = "crate::domain::deserialize_naive")]
    date: NaiveDate,
    content: String,
}

#[derive(Deserialize)]
struct DeleteEventParams {
    user_id: usize,
}

pub async fn validate_create_event_params(
    request: Request<Body>,
    next: Next,
) -> Result<axum::response::Response, (StatusCode, Json<Value>)> {
    let (parts, body) = request.into_parts();
    let body = match body::to_bytes(body, usize::MAX).await {
        Ok(body) => body,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error" : error.to_string()})),
            ))
        }
    };
    let payload: CreateEventParams = match serde_json::from_slice(&body) {
        Ok(params) => params,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error" : error.to_string()})),
            ))
        }
    };
    if payload.user_id == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "user_id must be greater than 0"})),
        ));
    }
    let request = Request::from_parts(parts, body::Body::from(body));
    Ok(next.run(request).await)
}
pub async fn validate_events_for_params(
    request: Request<Body>,
    next: Next,
) -> Result<axum::response::Response, (StatusCode, Json<Value>)> {
    let (parts, body) = request.into_parts();
    let body = match body::to_bytes(body, usize::MAX).await {
        Ok(body) => body,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error" : error.to_string()})),
            ))
        }
    };
    let payload: CreateEventParams = match serde_json::from_slice(&body) {
        Ok(params) => params,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error" : error.to_string()})),
            ))
        }
    };
    if payload.user_id == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "user_id must be greater than 0"})),
        ));
    }
    let request = Request::from_parts(parts, body::Body::from(body));
    Ok(next.run(request).await)
}
