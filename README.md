# Hanzo Extract

[![Crates.io](https://img.shields.io/crates/v/hanzo-extract.svg)](https://crates.io/crates/hanzo-extract)
[![Documentation](https://docs.rs/hanzo-extract/badge.svg)](https://docs.rs/hanzo-extract)
[![CI](https://github.com/hanzoai/extract/actions/workflows/ci.yml/badge.svg)](https://github.com/hanzoai/extract/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Content extraction with built-in sanitization for LLM applications.

## Features

- **Web Extraction**: Fetch and extract clean text from web pages
- **PDF Extraction**: Extract text from PDF documents
- **Conversation Extraction**: Export Claude Code sessions for training datasets
- **Sanitization**: Automatic PII redaction via [hanzo-guard](https://github.com/hanzoai/guard)

## Installation

```bash
cargo add hanzo-extract
```

Or add to `Cargo.toml`:

```toml
[dependencies]
hanzo-extract = "0.1"
```

## Quick Start

### Web Extraction

```rust
use hanzo_extract::{WebExtractor, ExtractorConfig, Extractor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = WebExtractor::new(ExtractorConfig::default());
    let result = extractor.extract("https://example.com").await?;

    println!("Title: {:?}", result.title);
    println!("Text: {}", result.text);
    println!("Words: {}", result.word_count);

    Ok(())
}
```

### PDF Extraction

```rust
use hanzo_extract::{PdfExtractor, Extractor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = PdfExtractor::default();
    let result = extractor.extract("document.pdf").await?;

    println!("Text: {}", result.text);
    Ok(())
}
```

### Conversation Extraction

Extract Claude Code conversations for AI training:

```rust
use hanzo_extract::conversations::{ConversationExporter, ExporterConfig};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut exporter = ConversationExporter::new();

    exporter.export(
        Path::new("~/.claude/projects"),
        Path::new("./training-data"),
    )?;

    Ok(())
}
```

## CLI Tools

### extract-web

```bash
# Install
cargo install hanzo-extract --features web

# Usage
extract-web https://example.com
extract-web https://example.com --json
```

### extract-conversations

```bash
# Install
cargo install hanzo-extract --features conversations

# Usage
extract-conversations --source ~/.claude/projects --output ./conversations

# Options
extract-conversations --help
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `web` | Yes | Web page extraction |
| `pdf` | Yes | PDF document extraction |
| `sanitize` | Yes | PII redaction via hanzo-guard |
| `conversations` | No | Claude Code conversation extraction |

```toml
# Minimal (no extraction)
hanzo-extract = { version = "0.1", default-features = false }

# Web only
hanzo-extract = { version = "0.1", default-features = false, features = ["web"] }

# Full (all features)
hanzo-extract = { version = "0.1", features = ["full"] }
```

## Conversation Export

The conversation extractor creates training datasets from Claude Code sessions:

```
~/.claude/projects/
├── project-a/
│   ├── session1.jsonl
│   └── session2.jsonl
└── project-b/
    └── session.jsonl

↓ extract-conversations

./conversations/
├── conversations_20251222.jsonl  # Full data
├── training_20251222.jsonl       # Instruction/response format
└── splits/
    ├── train_20251222.jsonl      # 80%
    ├── val_20251222.jsonl        # 10%
    └── test_20251222.jsonl       # 10%
```

### Features

- Extracts user/assistant conversation turns
- Anonymizes paths, secrets, emails, API keys
- Calculates quality scores (0.0-1.0)
- Creates reproducible train/val/test splits

### Quality Scoring

Conversations are scored based on:
- Thinking/reasoning presence (+0.2)
- Tool usage (+0.15)
- Agentic tools (Task, dispatch) (+0.1)
- Opus/Sonnet model (+0.1/+0.05)
- Response length (+0.1)

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Source    │ ──► │  Extractor   │ ──► │  Hanzo Guard    │
│ (URL/PDF)   │     │ (Text Parse) │     │ (Sanitization)  │
└─────────────┘     └──────────────┘     └─────────────────┘
                                                  │
                                                  ▼
                                         ┌─────────────────┐
                                         │  Clean Output   │
                                         │ (LLM-Ready)     │
                                         └─────────────────┘
```

## License

Dual licensed under MIT OR Apache-2.0.

## Related

- [hanzo-guard](https://github.com/hanzoai/guard) - LLM I/O sanitization
- [Hanzo AI](https://hanzo.ai) - AI infrastructure platform
