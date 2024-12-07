//! This module contains message buses, which are effectively just mpsc sender/receiver wrappers.

use tokio::sync::mpsc::{error::SendError, Receiver, Sender};
use std::fmt::Debug;

/// A handle for sending messages of type T to a module.
#[derive(Debug)]
pub struct MessageSender<T: Debug> {
    sender: Sender<T>
}

impl<T: Debug> MessageSender<T> {
    /// Initialize the message sender.
    pub fn new(sender: Sender<T>) -> Self {
        Self { sender }
    }

    /// Send a message through the sender.
    #[tracing::instrument(skip(self))]
    pub async fn send(&mut self, message: T) -> Result<(), SendError<T>> {
        self.sender
            .send(message)
            .await
    }
}   

impl<T: Debug> Clone for MessageSender<T> {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone() }
    }
}

/// A handle for a module to receive messages of type T.
#[derive(Debug)]
pub struct MessageReceiver<T: Debug> {
    receiver: Receiver<T>
}

impl <T: Debug> MessageReceiver<T> {
    /// Instantiate the message receiver.
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    /// Receive a message through the receiver.
    #[tracing::instrument(skip(self))]
    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}