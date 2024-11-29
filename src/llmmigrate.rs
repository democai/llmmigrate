use anthropic::client::Client;
use anthropic::config::AnthropicConfig;
use anthropic::types::{ContentBlock, Message, MessagesRequestBuilder, Role};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct LlmMigrate {
    client: Client,
    model: String,
    verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MigrationResponse {
    migrated_code: String,
    explanation: Option<String>,
}

impl LlmMigrate {
    pub fn new(api_key: &str, model: &str, verbose: bool) -> Result<Self> {
        let mut cfg = AnthropicConfig::new()?;
        cfg.api_key = api_key.to_string();
        let client = Client::try_from(cfg)?;

        Ok(LlmMigrate {
            client,
            model: model.to_string(),
            verbose,
        })
    }

    pub async fn migrate_code(
        &self,
        example_source: &str,
        current_source: &str,
        instructions: Option<&str>,
        destination: &Path,
    ) -> Result<()> {
        if self.verbose {
            println!("Starting code migration process...");
        }

        let system_message = Message {
            role: Role::Assistant,
            content: vec![ContentBlock::Text { 
                text: "You are an expert code migration assistant. Your task is to transform the provided source code into a new format based on an example and any additional instructions. Focus on:

1. Maintaining the core functionality while adopting the new style/patterns
2. Following best practices from the example code
3. Copy the format and structure of the example code
4. Preserving important comments and documentation
5. Ensuring the migrated code is complete and valid
6. Provide the full output in the migrated_code field.

Respond with a JSON object containing:
{
    \"migrated_code\": string,
    \"explanation\": string | null
}".into()
            }],
        };

        let user_message = Message {
            role: Role::User,
            content: vec![ContentBlock::Text { 
                text: format!(
                    "Migrate the following code:\n\n\
                    EXHIBIT A (Example of desired output format):\n\
                    ```\n{}\n```\n\n\
                    EXHIBIT B (Current code to migrate):\n\
                    ```\n{}\n```\n\n\
                    Additional Instructions:\n{}\n\n\
                    Please migrate Exhibit B to match the style and patterns shown in Exhibit A, \
                    following any additional instructions provided.",
                    example_source,
                    current_source,
                    instructions.unwrap_or("No additional instructions provided.")
                ).into()
            }],
        };

        if self.verbose {
            println!("Sending request to Claude...");
        }

        let messages_request = MessagesRequestBuilder::default()
            .messages(vec![system_message, user_message])
            .model(self.model.clone())
            .max_tokens(8192usize)
            .build()?;

        let response = self.client.messages(messages_request).await?;
        let content = response.content.first().ok_or(anyhow::anyhow!("No response content"))?;
        let text = match content {
            ContentBlock::Text { text } => text,
            _ => return Err(anyhow::anyhow!("Unexpected response content type")),
        };

        let migration: MigrationResponse = serde_json::from_str(text)?;

        if self.verbose {
            if let Some(explanation) = &migration.explanation {
                println!("Migration explanation: {}", explanation);
            }
        }

        // Create parent directories if they don't exist
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the migrated code to the destination file
        fs::write(destination, migration.migrated_code)?;

        if self.verbose {
            println!("Successfully wrote migrated code to: {}", destination.display());
        }

        Ok(())
    }
}
