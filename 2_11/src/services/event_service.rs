// src/services/event_service.rs
use crate::domain; // Import the Repository trait
use crate::domain::interfaces::event_service;
use crate::domain::models::{Event, TimeRange};
use axum::async_trait;
use chrono::NaiveDate;
use std::error::Error;

pub struct EventService;
type Repository<E> = dyn domain::Repository<Error = E> + Sync + Send;

#[async_trait]
impl event_service::EventService for EventService {
    type Error = Box<dyn Error>;

    async fn create_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        content: String,
        repository: &Repository<Self::Error>,
    ) -> Result<(), Self::Error> {
        repository.create_event(user_id, content, date).await
    }

    async fn events_for(
        &self,
        range: TimeRange,
        user_id: i64,
        date: NaiveDate,
        repository: &Repository<Self::Error>,
    ) -> Result<Vec<Event>, Self::Error> {
        repository.events_for(range, user_id, date).await
    }

    async fn update_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        content: String,
        repository: &Repository<Self::Error>,
    ) -> Result<(), Self::Error> {
        repository.update_event(user_id, date, content).await
    }

    async fn delete_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        repository: &Repository<Self::Error>,
    ) -> Result<(), Self::Error> {
        repository.delete_event(user_id).await
    }
}
