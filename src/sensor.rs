use crate::types::Message;
use rand::Rng;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct Sensor {
    id: u64,
    interval: Duration,
}

impl Sensor {
    pub fn new(id: u64, interval: Duration) -> Sensor {
        Sensor { id, interval }
    }
    pub fn generate_and_push(&mut self, sender: &Sender<Message>) -> f64 {
        let mut rng = rand::thread_rng();
        loop {
            let value = rng.gen_range(0..10);
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros();
            let message = Message::new(0, value, timestamp);
            sender.send(message).unwrap();
            println!("Sensor {}: Pushed value: {}", self.id, value);
            sleep(self.interval);
        }
    }
}
