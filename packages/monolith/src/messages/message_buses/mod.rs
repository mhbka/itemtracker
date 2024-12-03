//! This module contains message buses.
//! The initial intention was to have a sender and receiver interface, then have concrete mpsc implementations
//! that can be changed in the future (for eg, HTTP/Redis for splitting into microservices).
//! 
//! However, if I do need to do that in the future, I can just create a module for handling such communication,
//! then use the mpsc message bus to communicate with the module.

use tokio::sync::mpsc::{error::SendError, Receiver, Sender};

/// A handle for sending messages of type T to a module.
pub struct MessageSender<T> {
    sender: Sender<T>
}

impl<T> MessageSender<T> {
    /// Initialize the message sender.
    pub fn new(sender: Sender<T>) -> Self {
        Self { sender }
    }

    /// Send a message through the sender.
    pub async fn send(&mut self, message: T) -> Result<(), SendError<T>> {
        self.sender
            .send(message)
            .await
    }
}

impl<T> Clone for MessageSender<T> {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone() }
    }
}

/// A handle for a module to receive messages of type T.
pub struct MessageReceiver<T> {
    receiver: Receiver<T>
}

impl <T> MessageReceiver<T> {
    /// Instantiate the message receiver.
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    /// Receive a message through the receiver.
    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}