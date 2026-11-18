use serenity::async_trait;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
}
