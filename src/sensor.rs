use crate::types::{Message};
use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::sync::mpsc::Sender;

pub struct Sensor {
    id: u64,
    interval: Duration,
    sequence_number: u64,
}

impl Sensor {
    pub fn new(id: u64, interval: Duration) -> Sensor {
        Sensor {
            id,
            interval,
            sequence_number: 0,
        }
    }
    pub fn generate_and_push(&mut self, sender: &Sender<Message>) -> f64 {
        let mut rng = rand::thread_rng();
        loop {
            let value = rng.gen::<f64>();
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros();
            let message = Message::new(self.sequence_number, value, timestamp as u64);
            sender.send(message).unwrap();
            self.sequence_number += 1;
            println!("Sensor {}: Pushed value: {}", self.id, value);
            sleep(self.interval);
        }
    }
}
