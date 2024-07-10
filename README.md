# Fog-Computing-2024 Prototyping Assignment
Code and documentation for Fog Computing 2024 @ TU Berlin 

# Documentation 

Our application is a simplified version of an IOT air quality surveillance system.
Three sensors capture environmental data (temperature, humidity, CO2) which are filtered by the
client for measurement errors or anomalies and then forwarded to the server, where 
a simplified machine learning inference is performed.
We specifically deal with the fog computing challenges of unreliable communication channels
and resource constraints. We also provide an example implementation of an offloading strategy of compute-heavy tasks 
into the cloud.

## Sensors
Our application consists of two binaries.
The [client](src/main.rs) binary launches three threads which simulate sensor activity. 
Sensor logic is implemented in [sensor.rs](src/sensor.rs). Each sensor will generate random values depending on its sensor type,
and will produce an invalid value with a probability also depending on its type. These values are sent to the client-side [buffer](src/buffer.rs)
via a Rust MPSC channel, which is an efficient intra-thread communication mechanism. The buffer
has a fixed capacity of messages in accordance with a limited resource edge environment. Once the buffer is full, 
it will start to average sensor values across sensor types to make space for new messages. It will reflect this
with a sampling count, indicating how often a sensor value has been sampled. Sensor values are then read 
from the buffer again via a Rust channel and processed by the client.

## Client
The client will filter each sensor value depending on a valid range for its type and reject sensor values that
fall outside of this valid range. The range simulates another edge component that performs computation close to the data
for which it does not require much resources or context. After filtering the data, the client will try to send messages from the buffer 
to the server. The client
will push back sequenced messages to the previously mentioned buffer if the server is not available or the connection is terminated.
Pushed back messages are handled exactly the same as unsequenced messages, and will eventually be averaged with new sensor values
should the connection remain interrupted.

## Server
The [server](src/server.rs) will perform a simple linear regression inference as a simulation for more complicated
machine learning workflows which would reasonably be offloaded to the cloud. To this end, it stores a regression coefficient
for each sensor value type and computes the weighted sum of the latest sensor values per type and an intercept.
Similar to the client, the server has a message queue to which it will add messages to be sent to the client after it performed the inference.
Because we modeled the server component 
being run on sufficiently powerful hardware found in a cloud environment, the server side queue has unlimited capacity. 

The client and server communicate directly via TCP. Because we were tasked with implementing reliable messaging, we decided to go
with TCP for our networking protocol, as this frees us from implementing TCP guarantees on top of an unreliable UDP stream.

## Why Rust?
We have implemented the prototype using Rust because Rust makes it easy to write safe and correct concurrent applications that perform well and
consume little resources. Rust would therefore also be suitable for real-world usage in resource-constrained edge nodes and for networking services in the cloud.
For simplicity, we have implemented the concurrency using OS threads (i.e. ```std::thread```).

## Message formats
We have implemented two message [formats](src/types.rs).
Each format consists at least of a sequence number, its content value, and a timestamp.
Sensor messages also have a sampling counter as previously mentioned and indicate the sensor type from which they originated.
Sequence numbers potentially allow the client and the server to verify that they receive messages as expected,
that no messages are sent twice, and that no messages have been lost.






