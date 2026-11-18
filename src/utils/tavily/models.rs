use serde::{Deserialize, Serialize};

// Requests

#[derive(Debug, Clone, Serialize)]
pub struct TavilyRequest {
    pub query: String,
    pub include_answer: bool,
}

// Responses

#[derive(Debug, Clone, Deserialize)]
pub struct TavilyResponse {
    pub query: String,
    pub answer: Option<String>,
    pub results: Vec<ResultElement>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultElement {
    pub content: String,
}
