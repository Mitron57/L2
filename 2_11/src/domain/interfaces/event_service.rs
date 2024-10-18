use crate::domain;
use crate::domain::models::{Event, TimeRange};
use axum::async_trait;
use chrono::NaiveDate;
use std::error::Error;

type Repository = dyn domain::Repository<Error = Box<dyn Error>> + Sync + Send;

#[async_trait]
pub trait EventService {
    type Error;

    async fn create_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        content: String,
        repository: &Repository,
    ) -> Result<(), Self::Error>;

    async fn events_for(
        &self,
        range: TimeRange,
        user_id: i64,
        date: NaiveDate,
        repository: &Repository,
    ) -> Result<Vec<Event>, Self::Error>;

    async fn update_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        content: String,
        repository: &Repository,
    ) -> Result<(), Self::Error>;

    async fn delete_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        repository: &Repository,
    ) -> Result<(), Self::Error>;
}
