# Fog-Computing-2024 Prototyping Assignment
Code and documentation for Fog Computing 2024 @ TU Berlin 

# Design 

Our application specifically deals with the fog computing challenges of unreliable communication channels
and resource constraints. It also provides a simple implementation of an offloading strategy of compute-heavy tasks 
into the cloud.

Our application consists of three components which are implemented in two binaries.
The [client](src/client.rs) simulates sensor activity, generates messages from the generated sensor values
and sends these messages to the server. Sensor logic is implemented in [sensor.rs](src/sensor.rs), 
each sensor runs in its own thread. The application can handle an arbitrary amount of sensor threads.

The [server](src/server.rs) will average the messages it receives and send
a message with the computed average to the client. Messages are defined in [types.rs](src/types.rs).

The communication between client and sensors and client and server respectively is made reliable
by buffering messages with a number of FIFO queues. 
Because we modeled the client as representing a resource constrained
edge node, the [buffer](src/buffer.rs) it uses for the sensor values received has a fixed configurable capacity.
Once the capacity is exceeded in the buffer, the buffer will merge the oldest messages it has received
by averaging their values. This sampling strategy ensures that there is always space available in the buffer, and that
sensor measurements are never completely lost. Rather, sensor measurements can be roughly recovered
from the averaged values. To this end, we include a sampling value in the message indicating the number of times
it has been sampled.

For the server, we have implemented an incoming and outgoing queue. Received messages 
are stored in the incoming queue, from which we then read depending on the sliding window selected for the averaging.
Messages are then stored in the outgoing queue, from which we read first when we want to send messages back to the client, 
ensuring that messages are not lost on a disconnect or crash. Because we modeled the server component 
being run on sufficiently powerful hardware found in a cloud environment, the server side queues have unlimited capacity. 

We have implemented the prototype using Rust because Rust makes it easy to write safe and correct concurrent applications that perform well and
consume little resources. Rust would therefore also be suitable for real-world usage in resource-constrained edge nodes and for networking services in the cloud.
For simplicity, we have implemented the concurrency using OS threads (i.e. ```std::thread```).

Our simple message type consists of a sequence number, timestamp, integer value content, and a sampling counter as
previously discussed. Sequence numbers potentially allow the server and the client to verify if any messages are missing,
or if a message has been sent twice. The sampling counter indicates how often a message has been averaged with another message.
Sensors return randomly generated integer value representing
sensor measurements. Sensor threads communicate via Rust channels, which are built on top of efficient lock-free
queues and support a variety of messaging paradigms, making thread-to-thread communication simple and straightforward. 

The client and server communicate directly via TCP. Because we were tasked with implementing reliable messaging, we decided to go
with TCP for our networking protocol, as this frees us from implementing TCP guarantees on top of an unreliable UDP stream.




