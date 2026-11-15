use reqwest::Client;
use serenity::async_trait;

use crate::llm::{
    gemini::models::{
        GeminiRequest, GeminiResponse, GenerationConfig, Parameters, Properties, Query, Tool,
    },
    llm_provider::LLMProvider,
    system_prompt::build_system_prompt,
    tools::search_internet::search_internet,
};

pub struct GeminiClient {
    pub token: String,
    pub tavily_token: String,
    pub client: Client,
}
impl GeminiClient {
    pub fn new(token: String, tavily_token: String, client: Client) -> Self {
        Self {
            token,
            tavily_token,
            client,
        }
    }
}
impl GeminiClient {
    async fn send_request_to_llm(
        &self,
        prompt: &str,
        use_tools: bool,
    ) -> anyhow::Result<GeminiResponse> {
        let tools = if use_tools {
            vec![Tool {
                tool_type: String::from("function"),
                name: String::from("search_internet"),
                description: String::from(
                    "Searches the internet for current events, recent news, or real-time information.",
                ),
                parameters: Parameters {
                    parameters_type: String::from("object"),
                    properties: Properties {
                        query: Query {
                            query_type: String::from("string"),
                            description: String::from(
                                "The search query to look up on the internet.",
                            ),
                        },
                    },
                    required: vec![String::from("query")],
                },
            }]
        } else {
            vec![]
        };

        let gemini_request = GeminiRequest {
            model: String::from("gemini-3.5-flash-lite"),
            input: prompt.to_string(),
            system_instruction: build_system_prompt(),
            generation_config: GenerationConfig {
                max_output_tokens: 400,
            },
            tools,
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
        Ok(gemini_response)
    }

    fn extract_model_output(response: GeminiResponse) -> Option<String> {
        for step in response.steps {
            if step.step_type == "model_output"
                && let Some(contents) = step.content
                && let Some(content) = contents.into_iter().next()
            {
                return Some(content.text);
            }
        }
        None
    }
}

#[async_trait]
impl LLMProvider for GeminiClient {
    async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let gemini_response = self.send_request_to_llm(prompt, true).await?;

        for step in gemini_response.steps {
            match step.step_type.as_str() {
                "function_call" => match step.name.as_deref() {
                    Some("search_internet") => {
                        if let Some(args) = step.arguments {
                            let search_result: String =
                                search_internet(&self.client, &self.tavily_token, &args.query)
                                    .await?;
                            let second_prompt = format!(
                                "User question: {prompt}\n\nInternet search results:\n{search_result}"
                            );
                            let second_response =
                                self.send_request_to_llm(&second_prompt, false).await?;

                            if let Some(text) = Self::extract_model_output(second_response) {
                                return Ok(text);
                            }
                        }
                    }
                    Some(other) => {
                        anyhow::bail!("Unknown function called by Gemini: {other}");
                    }
                    None => {
                        anyhow::bail!("Function call step missing function name");
                    }
                },
                "model_output" => {
                    if let Some(contents) = step.content
                        && let Some(content) = contents.into_iter().next()
                    {
                        return Ok(content.text);
                    }
                }
                _ => {}
            }
        }
        anyhow::bail!("Error in generate response with gemini")
    }
}
