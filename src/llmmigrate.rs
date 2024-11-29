use anthropic::client::ClientBuilder;
use anthropic::types::CompleteRequestBuilder;
use anthropic::{AI_PROMPT, HUMAN_PROMPT};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct LlmMigrate {
    client: anthropic::client::Client,
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
        let client = ClientBuilder::default()
            .api_key(api_key.to_string())
            .build()?;

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

        let system_instructions = "You are an expert code migration assistant. Your task is to transform the provided source code into a new format based on an example and any additional instructions. Focus on:

1. Maintaining the core functionality while adopting the new style/patterns
2. Following best practices from the example code
3. Preserving important comments and documentation
4. Ensuring the migrated code is complete and valid

Respond with a JSON object containing:
{
    \"migrated_code\": string,
    \"explanation\": string | null
}";

        let prompt = format!(
            "{HUMAN_PROMPT}{}\n\nMigrate the following code:\n\n\
            EXHIBIT A (Example of desired format):\n\
            ```\n{}\n```\n\n\
            EXHIBIT B (Current code to migrate):\n\
            ```\n{}\n```\n\n\
            Additional Instructions:\n{}\n\n\
            Please migrate Exhibit B to match the style and patterns shown in Exhibit A, \
            following any additional instructions provided.{AI_PROMPT}",
            system_instructions,
            example_source,
            current_source,
            instructions.unwrap_or("No additional instructions provided.")
        );

        if self.verbose {
            println!("Sending request to Claude...");
        }

        let complete_request = CompleteRequestBuilder::default()
            .prompt(prompt)
            .model(self.model.clone())
            .stream(false)
            .stop_sequences(vec![HUMAN_PROMPT.to_string()])
            .build()?;

        let response = self.client.complete(complete_request).await?;
        let migration: MigrationResponse = serde_json::from_str(&response.completion)?;

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
