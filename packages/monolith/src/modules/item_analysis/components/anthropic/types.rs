//! API-specific types are derived from the docs: https://docs.anthropic.com/en/api/messages
use serde::{Deserialize, Serialize};

/// The request form for querying Anthropic API.
/// 
/// **NOTE**: There are other optional parameters, but they're left out as we don't (currently) use them.
/// Check the docs for what they are.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnthropicRequestForm {
    pub model: String,
    pub max_tokens: usize,
    pub messages: Vec<AnthropicMessage>,
    pub system: String
}

/// A single message in the content of a Anthropic API request.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: Vec<AnthropicMessageContent>
}

/// The layout of content inside an Anthropic API request message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnthropicMessageContent {
    #[serde(rename = "type")] // API expects `type` but it's a keyword
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AnthropicImageMessageContent>
}   

/// Used to send image blocks in an Anthropic API message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnthropicImageMessageContent {
    #[serde(rename = "type")] // API expects `type` but it's a keyword
    pub source_type: String,
    pub media_type: String,
    pub data: String
}

/// The response received from a Anthropic API request.
/// 
/// **NOTE**: There is other data returned in the response, but we don't (currently) use it,
/// so we just leave them out. Check the docs for what they are.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnthropicResponse {
    pub id: String,
    pub usage: AnthropicUsage,
    pub content: Vec<AnthropicMessageContent>,
}

/// The usage data for this query.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnthropicUsage {
    pub input_tokens: usize,
    pub output_tokens: usize
}

/// The format that the answers should be parsed into.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EvaluationAnswers {
    pub answers: Vec<String>
}
