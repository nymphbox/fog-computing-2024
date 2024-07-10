use crate::types::{AirQualityMessage, SensorMessage, SensorType};

use std::collections::{HashMap, VecDeque};
use std::io::{BufReader, Write};
use std::net::TcpListener;
use std::time::SystemTime;

mod types;

pub struct Server {
    address: String,
    sensor_data: HashMap<SensorType, i64>,
    outgoing_buffer: VecDeque<AirQualityMessage>,
    sequence: u64,
    coefficients: HashMap<SensorType, f32>,
    intercept: f32,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server {
            address,
            sensor_data: HashMap::new(),
            outgoing_buffer: VecDeque::new(),
            sequence: 0,
            coefficients: HashMap::from([
                (SensorType::Temperature, 0.1),
                (SensorType::Humidity, 0.2),
                (SensorType::CO2, 0.3),
            ]),
            intercept: 1.0,
        }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server listening on {}", self.address);
        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut reader = BufReader::new(&mut stream);
                    let result: Result<SensorMessage, _> = bincode::deserialize_from(&mut reader);
                    match result {
                        Ok(received_message) => {
                            println!("Server received message {:?}", received_message);
                            self.sensor_data
                                .insert(received_message.sensor_type, received_message.content);

                            if self.has_sensor_data() {
                                let mut sum = 0.0;
                                for (&sensor_type, &data_value) in self.sensor_data.iter() {
                                    if let Some(&beta) = self.coefficients.get(&sensor_type) {
                                        sum += data_value as f32 * beta;
                                    }
                                }
                                let inference = sum + self.intercept;
                                let air_quality = inference.round() as i64;
                                let timestamp = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_micros();
                                let response_message =
                                    AirQualityMessage::new(self.sequence, air_quality, timestamp);
                                self.sequence += 1;
                                self.outgoing_buffer.push_back(response_message);
                                if let Some(message) = self.outgoing_buffer.pop_front() {
                                    let encoded_message = bincode::serialize(&message).unwrap();
                                    match stream.write_all(&encoded_message) {
                                        Ok(_) => {
                                            println!(
                                                "Server: Sent message to client: {:?}",
                                                message
                                            );
                                        }
                                        Err(e) => {
                                            println!(
                                                "Server: Failed to send message to client: {}",
                                                e
                                            );
                                            self.outgoing_buffer.push_back(message);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Server: Failed to read from socket: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Server: Failed to accept connection: {}", e);
                }
            }
        }
    }
    fn has_sensor_data(&self) -> bool {
        [
            SensorType::Temperature,
            SensorType::Humidity,
            SensorType::CO2,
        ]
        .iter()
        .all(|sensor_type| self.sensor_data.contains_key(sensor_type))
    }
}

fn main() {
    let address = "0.0.0.0:8080".to_string();
    let mut server = Server::new(address);
    server.start()
}
