//! API-specific types are derived from the docs: https://docs.OpenAI.com/en/api/messages
//! 
//! **NOTE**: Some of these structs don't fully describe the actual data shapes,
//! leaving out data that we don't use. Check the docs for what they are
//! if you're expecting/need any of it.
use serde::{Deserialize, Serialize};

/// The request form for querying OpenAI API.
/// 
/// **NOTE**: There are other optional parameters, but they're left out as we don't (currently) use them.
/// Check the docs for what they are.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIRequestForm {
    pub model: String,
    pub max_completion_tokens: usize,
    pub messages: Vec<OpenAIMessage>
}

/// A single message in the content of a OpenAI API request.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: Vec<OpenAIMessageContent>
}

/// The content inside an OpenAI API request message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIMessageContent {
    #[serde(rename = "type")] // API expects `type` but it's a keyword
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<OpenAIImageURLMessage>
}   

/// Used to send image blocks in an OpenAI API message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIImageURLMessage {
    pub url: String
}

/// The response received from a OpenAI API request.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIResponse {
    pub id: String,
    pub usage: OpenAIUsage,
    pub choices: Vec<OpenAIMessageContent>,
}

/// The usage data for this query.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIUsage {
    pub prompt_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize
}

/// The message in an OpenAI API response.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIResponseMessage {
    pub message: OpenAIResponseMessageContent
}

/// The content of a message of an OpenAI API response.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIResponseMessageContent {
    role: String,
    content: String
}
