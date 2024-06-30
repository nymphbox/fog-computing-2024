mod client;
mod sensor;
mod types;

use crate::client::Client;
use crate::sensor::Sensor;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut sensor = Sensor::new(1, Duration::from_secs(1));

    let mut client = Client::new("127.0.0.1:16000".to_string());

    let sensor_task = thread::spawn(move || {
        sensor.generate_and_push(&sender);
    });

    let client_task = thread::spawn(move || {
        client.start(&receiver);
    });

    sensor_task.join().unwrap();
    client_task.join().unwrap();
}
