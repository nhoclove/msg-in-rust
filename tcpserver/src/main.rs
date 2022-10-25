use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::{
    fmt,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone)]
struct PubSubError;

impl fmt::Display for PubSubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in PubSub")
    }
}
pub struct Subscriber {
    pub stream: Arc<Mutex<TcpStream>>,
}

impl Subscriber {
    fn new(stream: Arc<Mutex<TcpStream>>) -> Subscriber {
        Subscriber { stream }
    }

    fn push(&self, data: &str) {
        self.stream
            .lock()
            .unwrap()
            .write_all(data.as_bytes())
            .unwrap();
    }
}

pub struct PubSub {
    pub subscribers: Vec<Subscriber>,
}

impl PubSub {
    fn new() -> PubSub {
        let mut subscribers: Vec<Subscriber> = Vec::new();

        PubSub { subscribers }
    }

    fn subscribe(&mut self, subscriber: Subscriber) -> Result<(), PubSubError> {
        self.subscribers.push(subscriber);
        Ok(())
    }

    fn publish(&self, data: &str) -> Result<(), PubSubError> {
        for subscriber in &self.subscribers {
            subscriber.push(data);
        }

        Ok(())
    }
}

const MESSAGE_SIZE: usize = 5;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port: 3000");
    let mut pubsub = PubSub::new();

    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();

        let sub_stream = Arc::new(Mutex::new(_stream));
        let subscriber = Subscriber::new(Arc::clone(&sub_stream));
        pubsub.subscribe(subscriber);

        println!("Connection established");

        // Store all the bytes for our received string
        let mut received: Vec<u8> = vec![];
        let mut rx_bytes = [0u8; MESSAGE_SIZE];
        loop {
            let bytes_read = sub_stream.lock().unwrap().read(&mut rx_bytes).unwrap();

            received.extend_from_slice(&rx_bytes);

            if bytes_read < MESSAGE_SIZE {
                break;
            }
        }

        let data = String::from_utf8(received)
            .map(|msg| msg)
            .expect("Error while concatenating string");

        // Write data to PubSub
        pubsub.publish(&data).unwrap();
        println!("data: {}", data);
    }
}
