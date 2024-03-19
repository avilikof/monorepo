use async_nats::{Client, ConnectError, PublishError, SubscribeError, Subscriber, jetstream};
use async_nats::jetstream::Context;
use async_nats::jetstream::context::CreateKeyValueError;
use async_nats::jetstream::kv::Store;
use async_nats::rustls::internal::msgs::enums::Compression::Deflate;
use bytes::Bytes;

/// Errors that might occur while using the `NatsClient`.
///
#[derive(Debug)]
pub enum NatsDriverError {
    /// Encapsulates errors that might arise during the subscription process.
    SubscribeError(SubscribeError),
    /// Indicates the absence of a subscriber in the current context.
    NoSubscriber,
    /// Wraps errors related to establishing a connection with the NATS server.
    ConnectionError(ConnectError),
    FailedToPublish(PublishError),
    FailedCreateJSBucket(CreateKeyValueError)
}

/// A client for interacting with NATS, supporting asynchronous connection and subscription.
#[derive(Debug)]
pub struct NatsStreamClient {
    /// The client connection to the NATS server.
    client: Client,
    /// An optional subscriber, available after a successful subscription.
    subscription: Option<Subscriber>,
}

pub struct NatsJetStreamClient {
    client: Client,
    js_storage: Option<Store> 
}

impl NatsStreamClient {
    /// Creates and connects a new `NatsClient` instance to a specified NATS server URL.
    ///
    /// # Arguments
    /// * `url` - A string slice that holds the URL of the NATS server to connect to.
    ///
    /// # Returns
    /// A new `NatsClient` instance connected to the specified NATS server.
    ///
    /// # Examples
    /// ```
    /// let client = NatsStreamClient::new("nats://localhost:4222").await;
    /// ```
    pub async fn new(url: &str) -> Self {
        Self {
            client: async_nats::connect(url).await.unwrap(),
            subscription: None,
        }
    }

    /// Subscribes to a specified subject.
    ///
    /// # Arguments
    /// * `subject` - The subject to subscribe to.
    ///
    /// # Returns
    /// `Ok(())` if subscription is successful, or `Err(NatsDriverError)` if an error occurs.
    ///
    /// # Examples
    /// ```
    /// match client.subscribe("updates").await {
    ///     Ok(_) => println!("Subscribed successfully"),
    ///     Err(e) => println!("Error subscribing: {:?}", e),
    /// }
    /// ```
    pub async fn subscribe(&mut self, subject: &str) -> Result<(), NatsDriverError> {
        match self.client.subscribe(subject.to_owned()).await {
            Err(err) => Err(NatsDriverError::SubscribeError(err)),
            Ok(subscriber) => {
                self.subscription = Some(subscriber);
                Ok(())
            }
        }
    }

    /// Publishes a message to a specified subject.
    ///
    /// This method asynchronously sends a message to the specified subject on the NATS server.
    /// It leverages the underlying `async_nats` clients publish functionality to achieve this.
    ///
    /// # Arguments
    /// * `subject` - A string slice that specifies the subject to which the message will be published.
    /// * `message` - The message to be published, encapsulated in a `Bytes` type, which allows for efficient transfer of binary data.
    ///
    /// # Returns
    /// `Ok(())` if the message is successfully published, or `Err(NatsDriverError::FailedToPublish(err))`
    /// if an error occurs during the publication process.
    ///
    /// # Errors
    /// This method can return a `NatsDriverError::FailedToPublish` if the underlying publish operation fails.
    ///
    /// # Examples
    /// `
    /// use bytes::Bytes; // Assuming `Bytes` type from `bytes` crate is used
    ///
    /// let message = Bytes::from("Hello, NATS!");
    /// match client.publish("greetings", message).await {
    ///     Ok(_) => println!("Message published successfully"),
    ///     Err(e) => println!("Failed to publish message: {:?}", e),
    /// }
    /// `
    ///
    /// Note: Before using this method, ensure that the client is properly connected to the NATS server.
    pub async fn publish(&self, subject: &str, message: Bytes) -> Result<(), NatsDriverError> {
        match self.client.publish(subject.to_owned(), message).await {
            Err(err) => Err(NatsDriverError::FailedToPublish(err)),
            Ok(_) => Ok(()),
        }
    }

    /// Retrieves the current subscriber, if any.
    ///
    /// # Returns
    /// A reference to the current `Subscriber` if available, or `Err(NatsDriverError::NoSubscriber)` if not.
    ///
    /// # Examples
    /// ```
    /// match client.get_subscriber() {
    ///     Ok(subscriber) => {
    ///         // Use subscriber for message handling
    ///     },
    ///     Err(e) => println!("Error: {:?}", e),
    /// }
    /// ```
    pub fn get_subscriber(&mut self) -> Result<&mut Subscriber, NatsDriverError> {
        match &mut self.subscription {
            None => Err(NatsDriverError::NoSubscriber),
            Some(subscriber) => Ok(subscriber),
        }
    }
}

impl NatsJetStreamClient {
    pub async fn new(url: &str) -> Self {
        Self {
            client: async_nats::connect(url).await.unwrap(),
            js_storage: None
        }
    }

    pub async fn create_kv_storage(&mut self, bucket_name: &str) -> Result<(), NatsDriverError> {
        let jstream = jetstream::new(self.client.clone());
        match jstream.create_key_value(jetstream::kv::Config{
            bucket: bucket_name.to_owned(),
            ..Default::default()
        }).await {
            Ok(store) => {
                self.js_storage = Option::from(store);
                Ok(())
            }
            Err(err) => {
               Err(NatsDriverError::FailedCreateJSBucket(err)) 
            }
        }
    }
    pub fn get_kv(&mut self) -> Option<&Store> {
        self.js_storage.as_ref()
    }
}