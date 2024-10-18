mod application;
mod domain;
mod repository;
mod services;

use crate::services::EventService;
use application::*;
use axum::middleware::AddExtension;
use axum::routing::{get, post};
use axum::{middleware, Router};
use repository::Postgres;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_uri = match std::env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("DATABASE_URL is not set");
            return Ok(());
        }
    };
    let postgres = Box::new(Postgres::new(&db_uri).await?);
    let service = Box::new(EventService);
    let app_state = Arc::new(AppState::new(postgres, service));
    let router = Router::new()
        .route(
            "/create_event",
            post(create_event).layer(middleware::from_fn(validate_events_for_params)),
        )
        .route("/update_event", post(update_event))
        .route(
            "/delete_event",
            post(delete_event).layer(middleware::from_fn(validate_create_event_params)),
        )
        .route("/events_for_day", get(events_for_day))
        .route("/events_for_week", get(events_for_week))
        .route("/events_for_month", get(events_for_month))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    Ok(axum::serve(listener, router).await?)
}
