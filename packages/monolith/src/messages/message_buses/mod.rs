//! This module contains message buses, which are effectively just mpsc sender/receiver wrappers.
use tokio::sync::{mpsc::{error::SendError, Receiver, Sender}, oneshot::error::RecvError};
use std::fmt::Debug;
use thiserror::Error;

/// The errors that may arise from failure to send/receive a message.
#[derive(Error, Debug, Clone)]
pub enum MessageError {
    #[error("{0}")]
    SendError(String),
    #[error("{0}")]
    RecvError(#[from] RecvError)
}

impl<T> From<SendError<T>> for MessageError {
    fn from(error: SendError<T>) -> Self {
        MessageError::SendError(error.to_string())
    }
}

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
    pub async fn send(&mut self, message: T) -> Result<(), MessageError> {
        self.sender
            .send(message)
            .await
            .map_err(Into::into)
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
    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}