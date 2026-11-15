use serde::{Deserialize, Serialize};

// POST Request

#[derive(Debug, Clone, Serialize)]
pub struct GeminiRequest {
    pub model: String,
    pub input: String,
    pub system_instruction: String,
    pub generation_config: GenerationConfig,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationConfig {
    pub max_output_tokens: i64,
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
