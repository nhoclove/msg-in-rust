mod pubsub {
    use std::fmt;

    #[derive(Debug, Clone)]
    struct PubSubError;

    impl fmt::Display for PubSubError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "error in PubSub")
        }
    }
    pub struct Subscriber;

    impl Subscriber {
        fn push(&self, data: &str) {
            println!("Get data: {}", data);
        }
    }

    pub struct PubSub {
        pub subscribers: Vec<Subscriber>,
    }

    impl PubSub {
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
}
