use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct EventParams {
    pub(crate) user_id: i64,
    #[serde(deserialize_with = "crate::domain::deserialize_naive")]
    pub date: NaiveDate,
}
