mod llmmigrate;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use crate::llmmigrate::LlmMigrate;

/// Semantic code search using local LLMs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to example source file (Exhibit A)
    #[arg(long)]
    example: PathBuf,

    /// Path to current source file to migrate (Exhibit B)
    #[arg(long)]
    source: PathBuf,

    /// Path where migrated code should be written
    #[arg(long)]
    destination: PathBuf,

    /// Additional instructions for migration (optional)
    #[arg(long)]
    instructions: Option<String>,

    /// Anthropic API key
    #[arg(long, env = "ANTHROPIC_API_KEY")]
    api_key: String,

    /// LLM model to use (default: claude-3-sonnet-20240229)
    #[arg(long, default_value = "claude-3-sonnet-20240229")]
    model: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let example_source = std::fs::read_to_string(&args.example)?;
    let current_source = std::fs::read_to_string(&args.source)?;

    let llm_migrate = LlmMigrate::new(&args.api_key, &args.model, args.verbose)?;

    llm_migrate
        .migrate_code(
            &example_source,
            &current_source,
            args.instructions.as_deref(),
            &args.destination,
        )
        .await?;

    Ok(())
}
