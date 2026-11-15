use serde::{Deserialize, Serialize};

// POST Request

#[derive(Debug, Clone, Serialize)]
pub struct GeminiRequest {
    pub model: String,
    pub input: String,
}

// Response

#[derive(Debug, Clone, Deserialize)]
pub struct GeminiResponse {
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub content: Option<Vec<Content>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}
