pub mod web_backend;
pub mod scraper_scheduler;
pub mod scraper;
pub mod item_analysis;
pub mod img_classifier;
pub mod storage;

use std::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::oneshot;

/// Represents a message to a module.
/// 
/// This is used for one-way communication, 
/// such as passing a gallery from one stage in the pipeline to the next.
/// 
/// ## Note (to self?)
/// If you want to communicate that something in the pipeline has failed,
/// you should emit a log (and in the future, let some state-tracking module know?).
/// 
/// Don't use `ModuleMessageWithReturn` with a `Result` return type to represent
/// if the next module is successful.
#[derive(Debug)]
pub struct ModuleMessage<Message> 
where Message: Send + Sync + Serialize + DeserializeOwned + Clone + Debug {
    message: Message,
}

impl<Message> ModuleMessage<Message>
where Message: Send + Sync + Serialize + DeserializeOwned + Clone + Debug {
    /// Initialize the message, return it as well as the oneshot channel receiver.
    pub fn new(message: Message) -> Self {
        Self { message }
    }

    /// Obtain the actual message, consuming this `ModuleMessage`.
    pub fn get_msg(self) -> Message {
        self.message
    }
}


/// Generic struct for a message to a module that requires a response.
/// 
/// This is used for things like "cross-module" function calls,
/// such as fetching some data from a storage module.
/// 
/// ## Use
/// 
/// - `new()` returns this struct along with a **receiver** for receiving the response. 
/// - This struct is sent to an actor via something like an mpsc channel, which can use `get_msg()` to get the actual message data. 
/// - After acting on the message, the actor can pass a response to `respond()`, which the original function can receive by `await`ing the **receiver**.
/// 
/// ## Example
/// 
/// ```rust
/// WIP
/// ```
#[derive(Debug)]
pub struct ModuleMessageWithReturn<Message, Return>
where 
    Message: Send + Sync + Serialize + DeserializeOwned + Clone + Debug, 
    Return: Send + Sync + Serialize + DeserializeOwned + Clone + Debug
{
    message: Message,
    respond_to: oneshot::Sender<Return>
}

impl<Message, Return> ModuleMessageWithReturn<Message, Return>
where 
    Message: Send + Sync + Serialize + DeserializeOwned + Clone + Debug, 
    Return: Send + Sync + Serialize + DeserializeOwned + Clone + Debug
{   
    /// Initialize the message, return it as well as the oneshot channel receiver.
    pub fn new(message: Message) -> (Self, oneshot::Receiver<Return>) {
        let (sender, receiver) = oneshot::channel();
        let message = Self { message, respond_to: sender };
        (message, receiver)
    }

    /// Obtain a clone of the message.
    pub fn get_msg(&self) -> Message {
        self.message.clone()
    }

    /// Attempt to send the response.
    pub fn respond(self, response: Return) -> Result<(), Return> {
        self.respond_to.send(response)
    }
}





