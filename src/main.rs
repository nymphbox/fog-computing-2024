mod buffer;
mod client;
mod sensor;
mod types;

use crate::client::Client;
use crate::sensor::Sensor;
use crate::types::SensorType;
use buffer::Buffer;
use std::time::Duration;
use std::{env, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server_address>", args[0]);
        std::process::exit(1);
    }
    let address = &args[1];

    let (buffer_tx, buffer_rx) = std::sync::mpsc::channel();
    let (send_tx, send_rx) = std::sync::mpsc::channel();
    let (confirm_tx, confirm_rx) = std::sync::mpsc::channel();

    let mut client = Client::new(address.to_string());

    let sensor_types = [SensorType::Temperature,
        SensorType::Humidity,
        SensorType::CO2];
    let mut buffer = Buffer::new(3, Duration::from_millis(100));
    let mut sensor_threads = vec![];
    for (sensor_id, sensor_type) in sensor_types.iter().enumerate() {
        let sender = buffer_tx.clone();
        let mut sensor = Sensor::new(sensor_id, Duration::from_secs(5), *sensor_type);
        let sensor_task = thread::spawn(move || {
            sensor.generate_and_push(&sender);
        });
        sensor_threads.push(sensor_task);
    }

    let buffer_task = thread::spawn(move || {
        buffer.start(&buffer_rx, &send_tx, &confirm_rx);
    });

    let client_task = thread::spawn(move || {
        client.start(&send_rx, &confirm_tx);
    });

    for task in sensor_threads {
        task.join().unwrap();
    }

    client_task.join().unwrap();
    buffer_task.join().unwrap();
}
