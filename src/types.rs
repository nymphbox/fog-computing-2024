use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub sequence_number: u64,
    pub timestamp: u64,
    pub content: f64,
}

impl Message {
    pub fn new(sequence_number: u64, content: f64, timestamp: u64) -> Message {
        Message {
            sequence_number,
            content,
            timestamp,
        }
    }
}
