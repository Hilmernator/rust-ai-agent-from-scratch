use super::adapter::ModelAdapter;
use super::types::*;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

pub struct AnthropicAdapter {
    api_key: String,
    client: Client,
}

impl AnthropicAdapter {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

impl ModelAdapter for AnthropicAdapter {
    async fn send(&self, request: ModelRequest) -> Result<ModelResponse> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;
        let anthropic_resp = res.json::<AnthropicResponse>().await?;
        match anthropic_resp.stop_reason.as_str() {
            "end_turn" => {
                let block = anthropic_resp
                    .content
                    .iter()
                    .find(|b| matches!(b, AnthropicContent::Text { .. }))
                    .ok_or_else(|| anyhow::anyhow!("no text block in response"))?;
                if let AnthropicContent::Text { text } = block {
                    Ok(ModelResponse::FinalAnswer(text.clone()))
                } else {
                    unreachable!()
                }
            }
            "tool_use" => {
                let block = anthropic_resp
                    .content
                    .iter()
                    .find(|b| matches!(b, AnthropicContent::ToolUse { .. }))
                    .ok_or_else(|| anyhow::anyhow!(" no tool cal in response"))?;
                if let AnthropicContent::ToolUse { id, name, input } = block {
                    Ok(ModelResponse::ToolCall {
                        id: id.clone(),
                        name: name.clone(),
                        args: input.clone(),
                    })
                } else {
                    unreachable!()
                }
            }
            _ => Err(anyhow::anyhow!(
                "unexpected stop_reason {}",
                anthropic_resp.stop_reason
            )),
        }
    }
}

#[derive(Deserialize)]
struct AnthropicResponse {
    id: String,
    r#type: String,
    role: String,
    content: Vec<AnthropicContent>,
    model: String,
    stop_reason: String,
    usage: Usage,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicContent {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
}

#[derive(Deserialize)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}
