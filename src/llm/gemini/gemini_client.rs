use crate::llm::llm_provider::LLMProvider;

pub struct GeminiClient {
    pub token: String,
    pub prompt: String,
}
impl GeminiClient {
    pub fn new(token: String, prompt: String) -> Self {
        Self { token, prompt }
    }
}
impl LLMProvider for GeminiClient {
    async fn generate(&self, prompt: String) {}
}
