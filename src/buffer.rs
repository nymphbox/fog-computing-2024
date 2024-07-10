use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

use crate::types::{Mergeable, SensorMessage, SensorType};

struct BufferSizeLimit {
    size_limit: usize,
    count: usize,
    oldest_index: Option<usize>
}

pub struct Buffer {
    buffer: VecDeque<SensorMessage>,
    timeout: Duration,
    limits: HashMap<SensorType, BufferSizeLimit>
}

impl Buffer {
    pub fn new(size_limit_per_type: usize, timeout: Duration) -> Self {
        // Ensure that the buffer size is at least two for the merge operation
        assert!(size_limit_per_type >= 2);
        Buffer {
            buffer: VecDeque::new(),
            timeout,
            limits: SensorType::iterator().map(|&sensor_type|(sensor_type, BufferSizeLimit {
                size_limit: size_limit_per_type,
                count: 0,
                oldest_index: None,
            })).collect()
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
                    if let Some(limit) = self.limits.get_mut(&element.sensor_type) {
                        println!("Client: Adding {:?} to queue", element);
                        self.buffer.push_back(element);
                        limit.count += 1;
                        if limit.oldest_index.is_none() {
                            limit.oldest_index = Some(self.buffer.len() - 1)
                        }
                        if limit.count >= limit.size_limit { //if this is true there are atleast 2 elements of the same type in the buffer
                            let oldest_index = limit.oldest_index.expect("msg");
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
                    } else {
                        println!("Client: Dismissing data from unknown sensor type")
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
            if let Some(element) = self.buffer.front().cloned() {
                match send_tx.send(element) {
                    Ok(_) => {
                        match confirm_rx
                            .recv()
                            .expect("Client: Connection handler crashed during message send")
                        {
                            true => {
                                _ = self.buffer.pop_front();
                                if let Some(limit) = self.limits.get_mut(&element.sensor_type) {
                                    limit.count -= 1;
                                    limit.oldest_index = self.buffer.iter().position(|x| x.sensor_type == element.sensor_type)
                                }
                                //We only add known sensor types to the buffer
                            },
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
