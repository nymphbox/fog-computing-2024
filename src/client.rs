use crate::types::Message;

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
    pub fn start(&mut self, send_rx: &Receiver<Message>, confirm_tx: &Sender<bool>) {
        println!("Client: Starting...");
        loop {
            let message = send_rx.recv().unwrap();

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
            let result: Result<Message, _> = bincode::deserialize_from(&mut reader);
            if let Ok(received_message) = result {
                    println!("Client: Received {:?} from server", received_message);
            }
        }
    }
}
