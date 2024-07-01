use crate::types::Message;

use std::collections::VecDeque;
use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

pub struct Client {
    address: String,
    buffer: VecDeque<Message>,
    sequence_number: u64,
}

impl Client {
    pub fn new(address: String) -> Client {
        Client {
            address,
            buffer: VecDeque::new(),
            sequence_number: 0,
        }
    }
    pub fn start(&mut self, receiver: &Receiver<Message>) {
        println!("Client: Starting...");
        loop {
            let mut message = receiver.recv().unwrap();
            message.sequence_number = self.sequence_number;
            self.sequence_number += 1;
            println!("Client: Adding {:?} to queue", message);
            self.buffer.push_back(message);

            match TcpStream::connect(&self.address) {
                Ok(mut stream) => {
                    let message = self.buffer.pop_front().unwrap();
                    let serialized_message = bincode::serialize(&message).unwrap();
                    match stream.write_all(&serialized_message) {
                        Ok(_) => {
                            println!("Client: Sent {:?} to server", message);
                        }
                        Err(e) => {
                            println!("Client: Failed to send message {:?}to server: {}", message, e);
                            println!("Client: Adding message {:?} to queue", message);
                            self.buffer.push_back(message);
                        }
                    }

                    let mut reader = BufReader::new(stream);
                    let result: Result<Message, _> = bincode::deserialize_from(&mut reader);
                    match result {
                        Ok(received_message) => {
                            println!(
                                "Client: Received {:?} from server",
                                received_message
                            );
                        }
                        Err(e) => {
                            println!("Client: Failed to receive message from server: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Client: Failed to connect to server: {}", e);
                }
            }
        }
    }
}
