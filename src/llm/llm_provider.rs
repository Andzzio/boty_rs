pub trait LLMProvider {
    async fn generate(&self, prompt: String);
}
