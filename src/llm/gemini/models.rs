use serde::{Deserialize, Serialize};

// POST Request

#[derive(Debug, Clone, Serialize)]
pub struct GeminiRequest {
    pub model: String,
    pub input: String,
    pub system_instruction: String,
    pub generation_config: GenerationConfig,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationConfig {
    pub max_output_tokens: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

#[derive(Debug, Clone, Serialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub parameters_type: String,
    pub properties: Properties,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Properties {
    pub query: Query,
}

#[derive(Debug, Clone, Serialize)]
pub struct Query {
    #[serde(rename = "type")]
    pub query_type: String,
    pub description: String,
}

// Response

#[derive(Debug, Clone, Deserialize)]
pub struct GeminiResponse {
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    #[serde(rename = "type")]
    pub step_type: String,
    pub content: Option<Vec<Content>>,
    // pub id: Option<String>,
    pub name: Option<String>,
    pub arguments: Option<SearchArguments>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SearchArguments {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}
