use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

use crate::types::{Mergeable, Sequenced};

pub struct Buffer<T> {
    buffer: VecDeque<T>,
    size_limit: usize,
    timeout: Duration,
    sequence_number: u64
}

impl<T> Buffer<T>
where
    T: Clone + Mergeable + Sequenced + std::fmt::Debug,
{
    pub fn new(size_limit: usize, timeout: Duration) -> Self {
        assert!(size_limit >= 2); //Our merge strategy requires atleast 2 elements
        Buffer {
            buffer: VecDeque::new(),
            size_limit,
            timeout,
            sequence_number: 0
        }
    }

    pub fn start(&mut self, buffer_rx: &Receiver<T>, send_tx: &Sender<T>, confirm_rx: &Receiver<bool> ) {
        loop {
            match buffer_rx.recv_timeout(self.timeout) {
                Ok(mut element) => {
                    println!("Client: Adding {:?} to queue", element);
                    element.set_sequence_number(self.sequence_number);
                    self.sequence_number += 1;
                    self.buffer.push_back(element);

                    if self.buffer.len() >= self.size_limit {
                        let oldest_index = 0;
                        let merged_element = self.buffer[oldest_index].merge(&self.buffer[oldest_index + 1]);
                        let old = self.buffer.remove(oldest_index);
                        println!("Client: Buffer limit reached: Merged {:?} and {:?} to {:?}", old, self.buffer[oldest_index], merged_element);
                        self.buffer[oldest_index] = merged_element;
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    // No message received, keep on working
                }
                Err(RecvTimeoutError::Disconnected) => {
                    println!("Client: Sensor buffer channel disconnected");
                    break;
                }
            }
            if let Some(element) = self.buffer.front(){
                match send_tx.send(element.clone()) {
                    Ok(_) => {
                        match confirm_rx.recv().expect("Client: Connection handler crashed during message send")  {
                            true => {
                                _ = self.buffer.pop_front()
                            }
                            false => {
                                println!("Client: Keep message {:?} in queue", element);
                                //TODO do anything else here?
                            }
                        }
                    }
                    Err(_) => {
                        println!("Client: Send Error");
                        //TODO error handling if client doesnt respond?
                    }
                }
            }
        }
    }
}