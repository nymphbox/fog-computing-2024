use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub sequence_number: u64,
    pub timestamp: u128,
    pub content: i64,
}

impl Message {
    pub fn new(sequence_number: u64, content: i64, timestamp: u128) -> Message {
        Message {
            sequence_number,
            content,
            timestamp,
        }
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timestamp_secs = (self.timestamp / 1_000_000) as i64;
        let timestamp_nanos = ((self.timestamp % 1_000_000) * 1_000) as u32;
        let naive_dt = NaiveDateTime::from_timestamp(timestamp_secs, timestamp_nanos);
        let dt: DateTime<Utc> = DateTime::from_utc(naive_dt, Utc);
        let dt_string = dt.format("%Y-%m-%dT%H:%M:%S%.6f").to_string();
        write!(
            f,
            "Message(seq={}, content={}, timestamp={})",
            self.sequence_number, self.content, dt_string,
        )
    }
}
