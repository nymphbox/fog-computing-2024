use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

pub trait Mergeable {
    fn merge(&self, other: &Self) -> Self;
}

pub trait Sequenced {
    fn get_sequence_number(&self) -> u64;
    fn set_sequence_number(&mut self, new_sequence_number: u64);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub sequence_number: u64,
    pub timestamp: u128,
    pub content: i64,
    pub sample_count: u32
}

impl Message {
    pub fn new(sequence_number: u64, content: i64, timestamp: u128) -> Message {
        Message {
            sequence_number,
            content,
            timestamp,
            sample_count: 1
        }
    }
}

impl Mergeable for Message {
    fn merge(&self, other: &Self) -> Self {
        let merged_content = ((self.content * self.sample_count as i64) + (other.content * other.sample_count as i64)) / (self.sample_count as i64 + other.sample_count as i64);
        Message {
            sequence_number: other.sequence_number,
            timestamp: other.timestamp,
            content: merged_content,
            sample_count: self.sample_count + other.sample_count,
        }
    }
}

impl Sequenced for Message {
    fn get_sequence_number(&self) -> u64 {
        return self.sequence_number
    }

    fn set_sequence_number(&mut self, new_sequence_number: u64) {
        self.sequence_number = new_sequence_number;
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
            "Message(seq={}, content={}, timestamp={}, sample_count={})",
            self.sequence_number,
            self.content,
            dt_string,
            self.sample_count
        )
    }
}
