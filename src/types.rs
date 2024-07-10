use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(dead_code)]
pub trait Mergeable {
    fn merge(&self, other: &Self) -> Self;
}
#[allow(dead_code)]
pub trait Sequenced {
    fn set_sequence_number(&mut self, new_sequence_number: u64);
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SensorType {
    Temperature,
    Humidity,
    CO2,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct SensorMessage {
    pub sequence_number: u64,
    pub timestamp: u128,
    pub content: i64,
    pub sensor_type: SensorType,
    pub sample_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AirQualityMessage {
    pub sequence_number: u64,
    pub timestamp: u128,
    pub content: i64,
}

impl AirQualityMessage {
    pub fn new(sequence_number: u64, content: i64, timestamp: u128) -> AirQualityMessage {
        AirQualityMessage {
            sequence_number,
            content,
            timestamp,
        }
    }
}

impl SensorMessage {
    pub fn new(
        sequence_number: u64,
        content: i64,
        timestamp: u128,
        sensor_type: SensorType,
    ) -> SensorMessage {
        SensorMessage {
            sequence_number,
            content,
            timestamp,
            sensor_type,
            sample_count: 1,
        }
    }
}

impl Mergeable for SensorMessage {
    fn merge(&self, other: &Self) -> Self {
        let merged_content = ((self.content * self.sample_count as i64)
            + (other.content * other.sample_count as i64))
            / (self.sample_count as i64 + other.sample_count as i64);
        SensorMessage {
            sequence_number: other.sequence_number,
            timestamp: other.timestamp,
            content: merged_content,
            sample_count: self.sample_count + other.sample_count,
            sensor_type: self.sensor_type,
        }
    }
}

impl Sequenced for SensorMessage {
    fn set_sequence_number(&mut self, new_sequence_number: u64) {
        self.sequence_number = new_sequence_number;
    }
}

impl fmt::Debug for SensorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timestamp_secs = (self.timestamp / 1_000_000) as i64;
        let timestamp_nanos = ((self.timestamp % 1_000_000) * 1_000) as u32;
        let dt = DateTime::from_timestamp(timestamp_secs, timestamp_nanos)
            .expect("Failed to create UTC DateTime from timestamp");
        let dt_string = dt.format("%Y-%m-%dT%H:%M:%S%.6f").to_string();
        write!(
            f,
            "SensorMessage(sensor={:?}, seq={}, content={}, timestamp={}, sample_count={})",
            self.sensor_type, self.sequence_number, self.content, dt_string, self.sample_count
        )
    }
}

impl fmt::Debug for AirQualityMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timestamp_secs = (self.timestamp / 1_000_000) as i64;
        let timestamp_nanos = ((self.timestamp % 1_000_000) * 1_000) as u32;
        let dt = DateTime::from_timestamp(timestamp_secs, timestamp_nanos)
            .expect("Failed to create UTC DateTime from timestamp");
        let dt_string = dt.format("%Y-%m-%dT%H:%M:%S%.6f").to_string();
        write!(
            f,
            "AirQualityMessage(seq={}, content={}, timestamp={})",
            self.sequence_number, self.content, dt_string
        )
    }
}
