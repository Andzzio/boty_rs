use reqwest::Client;

use crate::utils::tavily::models::{TavilyRequest, TavilyResponse};

pub async fn search_internet(
    client: &Client,
    tavily_token: &str,
    query: &str,
) -> anyhow::Result<String> {
    let request_body = TavilyRequest {
        query: query.to_string(),
        include_answer: true,
    };

    let res = client
        .post("https://api.tavily.com/search")
        .header("Authorization", format!("Bearer {tavily_token}"))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let error_body = res.text().await?;
        anyhow::bail!("Error tavily body: {error_body}");
    }

    let tavily_response: TavilyResponse = res.json().await?;

    let mut combined_results = String::new();

    if let Some(answer) = tavily_response.answer
        && !answer.is_empty()
    {
        combined_results.push_str(&format!("Answer: {answer}\n\n"));
    }

    for (i, result) in tavily_response.results.iter().enumerate() {
        combined_results.push_str(&format!("Result {}:\n{}\n\n", i + 1, result.content));
    }

    if combined_results.is_empty() {
        anyhow::bail!("No results found in Tavily search");
    }

    Ok(combined_results)
}
