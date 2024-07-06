use crate::types::Message;

use std::collections::VecDeque;
use std::io::{BufReader, Write};
use std::net::TcpListener;
use std::time::SystemTime;

mod types;

pub struct Server {
    address: String,
    incoming_buffer: VecDeque<Message>,
    outgoing_buffer: VecDeque<Message>,
    window_size: u32,
    sequence: u64,
}

impl Server {
    pub fn new(address: String, window_size: u32) -> Server {
        Server {
            address,
            incoming_buffer: VecDeque::new(),
            outgoing_buffer: VecDeque::new(),
            window_size,
            sequence: 0,
        }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server listening on {}", self.address);
        // TODO: implement averaging on full buffer from the back by folding messages until space is free
        // TODO: implement batching for the dead letter queue
        // TODO: deploy on google cloud vps
        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut reader = BufReader::new(&mut stream);
                    let result: Result<Message, _> = bincode::deserialize_from(&mut reader);
                    match result {
                        Ok(received_message) => {
                            println!("Server received message {:?}", received_message);
                            self.incoming_buffer.push_back(received_message);
                            if self.incoming_buffer.len() >= self.window_size as usize {
                                let average = self.compute_average();
                                let timestamp = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_micros();
                                let response_message =
                                    Message::new(self.sequence, average, timestamp);
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
    fn compute_average(&mut self) -> i64 {
        let mut sum = 0;
        let mut count = 0;

        for _ in 0..self.window_size {
            if let Some(message) = self.incoming_buffer.pop_front() {
                sum += message.content;
                count += 1;
            }
        }
        if count > 0 {
            sum / count
        } else {
            0
        }
    }
}

fn main() {
    let adress = "0.0.0.0:8080".to_string();
    let window_size = 5;
    let mut server = Server::new(adress, window_size);
    server.start()
}
