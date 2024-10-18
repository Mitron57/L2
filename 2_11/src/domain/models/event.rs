use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub user_id: i64,
    pub content: String,
    #[serde(
        deserialize_with = "deserialize_naive",
        serialize_with = "serialize_naive"
    )]
    pub date: NaiveDate,
}

pub(crate) fn deserialize_naive<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

fn serialize_naive<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = date.format("%Y-%m-%d").to_string();
    serializer.serialize_str(&date_str)
}
