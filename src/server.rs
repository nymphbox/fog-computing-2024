use std::ffi::c_double;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::raw::uid_t;
use std::time::SystemTime;
use bincode::Options;
use crate::types::Message;

mod types;

pub struct Server {
    address: String,
    buffer: Vec<Message>,
    window_size: u32,
    sequence: u64,
}

impl Server {
    pub fn new(address: String, window_size: u32) -> Server {
        Server {
            address,
            buffer: Vec::new(),
            window_size,
            sequence: 0,
        }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server listening on {}", self.address);
        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut reader = BufReader::new(&mut stream);
                    let result: Result<Message, _> = bincode::deserialize_from(&mut reader);
                    match result {
                        Ok(received_message) => {
                            println!("Server received message {:?}", received_message);
                            self.buffer.push(received_message);
                            if self.buffer.len() >= self.window_size as usize {
                                let average = self.compute_average();
                                let timestamp = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_micros();
                                let response_message = Message::new(
                                    self.sequence,
                                    average,
                                    timestamp as u64
                                );
                                self.sequence += 1;
                                let encoded_message = bincode::serialize(&response_message).unwrap();
                                stream.write_all(&*encoded_message).unwrap();
                                println!("Server wrote message {:?}", response_message);
                            }
                        },
                        Err(e) => {
                            println!("Failed to read from socket: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    }
    fn compute_average(&mut self) -> f64 {
        let sum = self.buffer.iter().fold(0.0, |acc, message| acc + message.content);
        let count = self.buffer.len() as f64;
        sum / count
    }
}

fn main() {
    let adress = "127.0.0.1:16000".to_string();
    let window_size = 2;
    let mut server = Server::new(adress, window_size);
    server.start()
}