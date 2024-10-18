use crate::domain::models::{Event, TimeRange};
use axum::async_trait;
use chrono::NaiveDate;

#[async_trait]
pub trait Repository {
    type Error;

    async fn create_event(
        &self,
        user_id: i64,
        content: String,
        date: NaiveDate,
    ) -> Result<(), Self::Error>;

    async fn events_for(
        &self,
        range: TimeRange,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, Self::Error>;

    async fn update_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        content: String,
    ) -> Result<(), Self::Error>;

    async fn delete_event(&self, user_id: i64) -> Result<(), Self::Error>;
}
