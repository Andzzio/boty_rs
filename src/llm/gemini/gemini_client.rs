use reqwest::Client;
use serenity::async_trait;

use crate::llm::{
    gemini::models::{GeminiRequest, GeminiResponse},
    llm_provider::LLMProvider,
};

pub struct GeminiClient {
    pub token: String,
    pub client: Client,
}
impl GeminiClient {
    pub fn new(token: String, client: Client) -> Self {
        Self { token, client }
    }
}
#[async_trait]
impl LLMProvider for GeminiClient {
    async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let gemini_request = GeminiRequest {
            model: String::from("gemini-3.5-flash-lite"),
            input: prompt.to_string(),
        };
        let res = self
            .client
            .post("https://generativelanguage.googleapis.com/v1beta/interactions")
            .header("x-goog-api-key", &self.token)
            .header("Content-Type", "application/json")
            .header("Api-Revision", "2026-05-20")
            .json(&gemini_request)
            .send()
            .await?;
        if !res.status().is_success() {
            let error_body = res.text().await?;
            anyhow::bail!("Error gemini body: {error_body}");
        }
        let gemini_response: GeminiResponse = res.json().await?;
        for step in gemini_response.steps {
            if let Some(contents) = step.content
                && let Some(content) = contents.into_iter().next()
            {
                return Ok(content.text);
            }
        }
        anyhow::bail!("Error in generate response with gemini")
    }
}
