pub mod state_tracker;
pub mod scraper_scheduler;
pub mod search_scraper;
pub mod item_scraper;
pub mod item_analysis;
pub mod item_embedder;
pub mod storage;

use std::{fmt::Debug, future::Future};
use tokio::sync::oneshot;

/// Generic struct for a message to a module that requires a response.
/// 
/// This is used for things like "cross-module" function calls,
/// such as fetching some data from a storage module.
/// 
/// ## Use
/// 
/// - A sender calls `new()`, returning this along with a *receiver* for receiving the response 
/// - The message is sent to an actor via a channel
/// - The actor calls `act` or `act_async` with a (async) closure, to act on the message data and return a response
/// - The sender `await`s the *receiver* to receive this response
/// 
/// ## Note
/// 
/// This should not be used for communicating the success/failure of an operation triggered by a message,
/// such as communicating failure of a search scrape back to the scheduler.
/// 
/// Such issues should be communicated to the state tracker module.
/// ```
#[derive(Debug)]
pub struct ModuleMessageWithReturn<Message, Return>
where 
    Message: Send + Sync + Clone + Debug, 
    Return: Send + Sync + Clone + Debug
{
    message: Message,
    respond_to: oneshot::Sender<Return>
}

impl<Message, Return> ModuleMessageWithReturn<Message, Return>
where 
    Message: Send + Sync + Clone + Debug, 
    Return: Send + Sync + Clone + Debug
{   
    /// Initialize the message, return it as well as the oneshot channel receiver.
    pub fn new(message: Message) -> (Self, oneshot::Receiver<Return>) {
        let (sender, receiver) = oneshot::channel();
        let message = Self { message, respond_to: sender };
        (message, receiver)
    }

    /// Act upon the message and provide a response to it.
    /// 
    /// Returns an `Err` with the response value if it couldn't be successfully delivered.
    pub fn act<F>(self, f: F) -> Result<(), Return>
    where 
        F: FnOnce(Message) -> Return {
        let response = f(self.message);
        self.respond_to.send(response)
    }

    /// Act asynchronously upon the message and provide a response to it.
    /// 
    /// Returns an `Err` with the response value if it couldn't be successfully delivered.
    pub async fn act_async<F, Fut>(self, f: F) -> Result<(), Return>
    where 
        F: FnOnce(Message) -> Fut,
        Fut: Future<Output = Return> {
        let response = f(self.message).await;
        self.respond_to.send(response)
    }
}





