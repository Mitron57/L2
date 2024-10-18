use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Report {
    pub elapsed: std::time::Duration,
    pub result: BTreeMap<char, usize>,
}

impl Serialize for Report {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let time = format!("{:.3} s", self.elapsed.as_secs_f64());
        let mut struct_serializer = serializer.serialize_struct("Report", 2)?;
        struct_serializer.serialize_field("elapsed", &time)?;
        struct_serializer.serialize_field("result", &self.result)?;
        struct_serializer.end()
    }
}
