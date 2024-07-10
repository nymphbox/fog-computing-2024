use crate::types::{SensorMessage, SensorType};
use rand::Rng;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct Sensor {
    id: usize,
    interval: Duration,
    sensor_type: SensorType,
}

impl Sensor {
    pub fn new(id: usize, interval: Duration, sensor_type: SensorType) -> Sensor {
        Sensor {
            id,
            interval,
            sensor_type,
        }
    }
    pub fn generate_and_push(&mut self, sender: &Sender<SensorMessage>) -> f64 {
        let mut rng = rand::thread_rng();
        loop {
            let value = match self.sensor_type {
                SensorType::CO2 => {
                    if rng.gen_bool(0.33) {
                        rng.gen_range(-100..400)
                    } else {
                        rng.gen_range(400..5000)
                    }
                }
                SensorType::Humidity => {
                    if rng.gen_bool(0.33) {
                        rng.gen_range(-100..0)
                    } else {
                        rng.gen_range(0..100)
                    }
                }
                SensorType::Temperature => {
                    if rng.gen_bool(0.5) {
                        rng.gen_range(-40..60)
                    } else {
                        rng.gen_range(-10..30)
                    }
                }
            };
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros();
            let message = SensorMessage::new(0, value, timestamp, self.sensor_type);
            sender.send(message).unwrap();
            println!("Sensor {}: Pushed value: {}", self.id, value);
            sleep(self.interval);
        }
    }
}
