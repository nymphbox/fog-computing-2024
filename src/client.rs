use crate::types::{AirQualityMessage, SensorMessage, SensorType};

use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub struct Client {
    address: String,
}

impl Client {
    pub fn new(address: String) -> Client {
        Client { address }
    }
    pub fn start(&mut self, send_rx: &Receiver<SensorMessage>, confirm_tx: &Sender<bool>) {
        println!("Client: Starting...");
        loop {
            let message = send_rx.recv().unwrap();
            let valid_sensor_range = self.sensor_value_range(message.sensor_type);
            if message.content < valid_sensor_range.0 as i64
                || message.content > valid_sensor_range.1 as i64
            {
                println!(
                    "Client: Received message with value outside valid range: {:?}",
                    message
                );
                _ = confirm_tx.send(true);
                continue;
            }

            let mut stream = match TcpStream::connect(&self.address) {
                Ok(stream) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(5)))
                        .unwrap();
                    stream
                }
                Err(e) => {
                    _ = confirm_tx.send(false);
                    println!("Client: Failed to connect to server: {}", e);
                    continue;
                }
            };
            let serialized_message = bincode::serialize(&message).unwrap();
            match stream.write_all(&serialized_message) {
                Ok(_) => {
                    println!("Client: Sent {:?} to server", message);
                    _ = confirm_tx.send(true);
                }
                Err(e) => {
                    println!(
                        "Client: Failed to send message {:?}to server: {}",
                        message, e
                    );
                    _ = confirm_tx.send(false);
                }
            }
            let mut reader = BufReader::new(stream);
            let result: Result<AirQualityMessage, _> = bincode::deserialize_from(&mut reader);
            if let Ok(received_message) = result {
                println!("Client: Received {:?} from server", received_message);
            }
        }
    }

    pub fn sensor_value_range(&self, sensor_type: SensorType) -> (i32, i32) {
        match sensor_type {
            SensorType::CO2 => (400, 5000),
            SensorType::Humidity => (0, 100),
            SensorType::Temperature => (-10, 50),
        }
    }
}
