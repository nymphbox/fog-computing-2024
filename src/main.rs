mod client;
mod sensor;
mod types;

use crate::client::Client;
use crate::sensor::Sensor;
use std::time::Duration;
use std::{env, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server_address>", args[0]);
        std::process::exit(1);
    }
    let address = &args[1];

    let (sender, receiver) = std::sync::mpsc::channel();
    let _sensor = Sensor::new(1, Duration::from_secs(5));

    let mut client = Client::new(address.to_string());

    let sensor_ids = vec![1, 2, 3];
    let mut sensor_threads = vec![];
    for sensor_id in sensor_ids {
        let sender = sender.clone();
        let mut sensor = Sensor::new(sensor_id, Duration::from_secs(5));
        let sensor_task = thread::spawn(move || {
            sensor.generate_and_push(&sender);
        });
        sensor_threads.push(sensor_task);
    }

    let client_task = thread::spawn(move || {
        client.start(&receiver);
    });

    for task in sensor_threads {
        task.join().unwrap();
    }

    client_task.join().unwrap();
}
