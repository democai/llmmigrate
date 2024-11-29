# LLM Migrate

LLM Migrate is an AI-powered code migration tool that uses Claude to help developers migrate code from one implementation to another by providing an example. It analyzes an example of previously migrated code (Exhibit A) and applies similar transformations to your current code (Exhibit B) to generate a migrated version.

## Features

- Example-based code migration using Claude's advanced language understanding
- Applies transformations to new code
- Supports additional migration instructions for fine-tuning
- Defaults to Claude 3.5 Sonnet for code analysis and transformation

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Anthropic API key for Claude access

## Installation

1. Clone the repository

```sh
git clone https://github.com/democai/llmmigrate.git
```

2. Navigate to the project directory:

```sh
cd llmmigrate
```

3. Build the project:

```sh
cargo build --release
```

4. Set up your Anthropic API key:

```sh
export ANTHROPIC_API_KEY='your-api-key-here'
```

## Usage

```sh
llm-migrate [OPTIONS] <EXAMPLE> <SOURCE> <DESTINATION>
```

Arguments:
- `<EXAMPLE>`     Path to example source file (Exhibit A) showing desired migration
- `<SOURCE>`      Path to current source file to migrate (Exhibit B)
- `<DESTINATION>` Path where migrated code should be written

Options:
- `-i, --instructions <INSTRUCTIONS>`  Additional instructions for migration
- `-m, --model <MODEL>`               LLM model to use (default: claude-3-5-sonnet-latest)
- `-v, --verbose`                     Enable verbose output
- `-h, --help`                        Print help
- `-V, --version`                     Print version

## Example

```sh
llm-migrate \
  --instructions "Be sure to preserve the original code's for loops" \
  example-output.jsx \
  src/components/MyComponent.jsx \
  src/components/MyNewComponent.jsx
```

## How It Works

1. The tool formulates a prompt using the example migration (Exhibit A), the current source code (Exhibit B), and any additional instructions provided.
2. It sends this prompt to the LLM to generate the migrated code.
3. The output from the LLM is then written to the specified destination path.

## License

[MIT License](LICENSE)
