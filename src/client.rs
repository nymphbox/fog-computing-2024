use std::collections::VecDeque;
use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use bincode::Options;
use crate::types::{Message};

pub struct Client {
    address: String,
    buffer: VecDeque<Message>,
}

impl Client {
    pub fn new(address: String) -> Client {
        Client {
            address,
            buffer: VecDeque::new(),
        }
    }
    pub fn read_sensor_message(&self, receiver: &Receiver<Message>) -> Message {
        let message = receiver.recv().unwrap();
        println!("Client: Read message with sequence number: {}", message.sequence_number);
        return message;
    }
    pub fn start(&mut self, receiver: &Receiver<Message>) {
        println!("Client: Starting...");
        loop {
            let message = self.read_sensor_message(&receiver);
            println!("Client: Read message: {:?}", message);

            match TcpStream::connect(&self.address) {
                Ok(mut stream) => {
                    let serialized_message = bincode::serialize(&message).unwrap();
                    match stream.write_all(&serialized_message) {
                        Ok(_) => {
                            println!("Client: Sent message to server: {:?}", message);
                        },
                        Err(e) => {
                            println!("Client: Failed to send message to server: {}", e);
                        }
                    }

                    let mut reader = BufReader::new(stream);
                    let result: Result<Message, _> = bincode::options().deserialize_from(&mut reader);
                    match result {
                        Ok(received_message) => {
                            println!("Client: Received message from server: {:?}", received_message);
                        },
                        Err(e) => {
                            println!("Client: Failed to receive message from server: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Client: Failed to connect to server: {}", e);
                }
            }
        }
    }
}