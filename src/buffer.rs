use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

use crate::types::{Mergeable, SensorMessage, Sequenced};

pub struct Buffer {
    buffer: VecDeque<SensorMessage>,
    size_limit: usize,
    timeout: Duration,
}

impl Buffer {
    pub fn new(size_limit: usize, timeout: Duration) -> Self {
        // Ensure that the buffer size is at least two for the merge operation
        assert!(size_limit >= 2);
        Buffer {
            buffer: VecDeque::new(),
            size_limit,
            timeout,
        }
    }

    pub fn start(
        &mut self,
        buffer_rx: &Receiver<SensorMessage>,
        send_tx: &Sender<SensorMessage>,
        confirm_rx: &Receiver<bool>,
    ) {
        loop {
            match buffer_rx.recv_timeout(self.timeout) {
                Ok(element) => {
                    println!("Client: Adding {:?} to queue", element);
                    self.buffer.push_back(element);

                    if self.buffer.len() >= self.size_limit {
                        let oldest_index = 0;
                        let oldest_sensor_type = self.buffer[oldest_index].sensor_type;

                        // Find the next message with the same sensor type
                        if let Some(next_index) = self.buffer.iter().position(|x| {
                            x.sensor_type == oldest_sensor_type && self.buffer[oldest_index] != *x
                        }) {
                            let merged_element =
                                self.buffer[oldest_index].merge(&self.buffer[next_index]);

                            // Remove the oldest message
                            let old = self
                                .buffer
                                .remove(oldest_index)
                                .expect("Can't remove from an empty buffer");

                            // Replace the next message with the merged message
                            self.buffer[next_index - 1] = merged_element; // Adjust index due to removal

                            println!(
                                "Client: Buffer limit reached: Merged {:?} and {:?} to {:?}",
                                old,
                                self.buffer[next_index - 1],
                                merged_element
                            );
                        }
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
            if let Some(element) = self.buffer.front() {
                match send_tx.send(*element) {
                    Ok(_) => {
                        match confirm_rx
                            .recv()
                            .expect("Client: Connection handler crashed during message send")
                        {
                            true => _ = self.buffer.pop_front(),
                            false => {
                                println!("Client: Keep message {:?} in queue", element);
                            }
                        }
                    }
                    Err(_) => {
                        println!("Client: Send Error");
                    }
                }
            }
        }
    }
}
