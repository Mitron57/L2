use crate::domain::interfaces::repository::Repository;
use crate::domain::models::{Event, TimeRange};
use axum::async_trait;
use chrono::NaiveDate;
use deadpool_postgres::{Client, Manager, Pool, Runtime, Timeouts};
use serde::__private::de::Content;
use std::error;
use tokio_postgres::{Config, NoTls};

pub struct Postgres {
    pool: Pool,
}

impl Postgres {
    pub async fn new(uri: &str) -> Result<Postgres, Box<dyn error::Error>> {
        let config = uri.parse::<Config>()?;
        let pool = Pool::builder(Manager::new(config, NoTls))
            .runtime(Runtime::Tokio1)
            .timeouts(Timeouts::wait_millis(30000))
            .build()?;
        Ok(Postgres { pool })
    }
}

#[async_trait]
impl Repository for Postgres {
    type Error = Box<dyn error::Error>;

    async fn create_event(
        &self,
        user_id: i64,
        content: String,
        date: NaiveDate,
    ) -> Result<(), Self::Error> {
        let conn = self.pool.get().await?;
        conn.execute(
            "INSERT INTO events (user_id, date, content) VALUES ($1, $2, $3)",
            &[&user_id, &date.to_string(), &content], // Convert NaiveDate to String
        )
        .await?;
        Ok(())
    }

    async fn events_for(
        &self,
        range: TimeRange,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, Self::Error> {
        let conn = self.pool.get().await?;
        let query = match range {
            TimeRange::DAY => "SELECT * FROM events WHERE user_id = $1 AND date = $2",
            TimeRange::WEEK => {
                "SELECT * FROM events WHERE user_id = $1 AND date >= $2 - INTERVAL '6 days'"
            }
            TimeRange::MONTH => {
                "SELECT * FROM events WHERE user_id = $1 AND date >= $2 - INTERVAL '29 days'"
            }
        };

        let rows = conn.query(query, &[&user_id, &date.to_string()]).await?; // Convert NaiveDate to String
        let events = rows
            .iter()
            .map(|row| Event {
                user_id: row.get("user_id"),
                content: row.get("content"),
                date: NaiveDate::parse_from_str(row.get::<_, &str>("date"), "%Y-%m-%d").unwrap(),
            })
            .collect();

        Ok(events)
    }

    async fn update_event(
        &self,
        user_id: i64,
        date: NaiveDate,
        content: String,
    ) -> Result<(), Self::Error> {
        let conn = self.pool.get().await?;
        conn.execute(
            "UPDATE events SET content = $1 WHERE user_id = $2 AND date = $3",
            &[&content, &user_id, &date.to_string()], // Convert NaiveDate to String
        )
        .await?;
        Ok(())
    }

    async fn delete_event(&self, user_id: i64) -> Result<(), Self::Error> {
        let conn = self.pool.get().await?;
        conn.execute("DELETE FROM events WHERE user_id = $1", &[&user_id])
            .await?;
        Ok(())
    }
}
