use crate::model::{
    adapter::ModelAdapter,
    anthropic::AnthropicAdapter,
    types::{ContentBlock, Message, MessageContent, ModelRequest, ModelResponse, Role, ToolSchema},
};
use crate::tools::GetCurrentTime;
use anyhow::Result;
use std::io::{self, Write};
mod model;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let api_key = std::env::var("ANTHROPIC_API_KEY")?;
    let adapter = AnthropicAdapter::new(api_key);
    let mut history: Vec<Message> = vec![];

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        }

        history.push(Message {
            role: Role::User,
            content: model::types::MessageContent::Text(input.to_string()),
        });

        loop {
            let request = ModelRequest {
                model: "claude-opus-4-6".to_string(),
                max_tokens: 1024,
                messages: history.clone(),
                tools: vec![ToolSchema {
                    name: "get_current_time".to_string(),
                    description: Some("Returns the current time on the local machine.".to_string()),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {}
                    }),
                }],
            };
            let response = adapter.send(request).await?;
            match response {
                ModelResponse::FinalAnswer(text) => {
                    println!("Claude: {}", text);
                    history.push(Message {
                        role: Role::Assistant,
                        content: MessageContent::Text(text),
                    });
                    break;
                }
                ModelResponse::ToolCall { id, name, args } => {
                    history.push(Message {
                        role: Role::Assistant,
                        content: MessageContent::Blocks(vec![ContentBlock::ToolUse {
                            id: id.clone(),
                            name: name.clone(),
                            input: args.clone(),
                        }]),
                    });
                    let result = match name.as_str() {
                        "get_current_time" => GetCurrentTime.execute(),
                        _ => format!("Unknown tool: {}", name),
                    };
                    history.push(Message {
                        role: Role::User,
                        content: MessageContent::Blocks(vec![ContentBlock::ToolResult {
                            tool_use_id: id,
                            content: result,
                        }]),
                    });
                }
            }
        }
    }

    Ok(())
}
