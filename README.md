# docpack

Markdown document packager for AI projects.

`docpack` scans Markdown documentation folders, analyzes document collections, and creates AI-ready Markdown packages with token-aware splitting.

## Overview

`docpack` is designed for preparing large Markdown documentation sets for AI workflows.

It provides:

- Documentation inspection
- Token estimation
- Duplicate detection
- Markdown packaging
- Token-based splitting
- Reproducible CLI workflows

## Features

- Recursive Markdown file discovery
- Markdown document loading
- Document merging with source tracking
- Token counting using OpenAI-compatible tokenizer
- Token-based splitting
- Duplicate content detection
- Inspect reports
- AI-ready package generation
- CLI workflow
- Release binary support

## Installation

Build from source:

```bash
cargo build --release
```

Binary:

```bash
target/release/docpack
```

## Commands

### Inspect documentation

Analyze a Markdown documentation directory:

```bash
docpack inspect \
  --input ./docs
```

Example output:

```text
Inspect Report
--------------
Files:      1435
Bytes:      ...
Tokens:     ...
Duplicates: ...
```

### Pack documentation

Create AI-ready Markdown chunks:

```bash
docpack pack \
  --input ./docs \
  --output ./merge/package \
  --max-tokens 12000
```

Output example:

```text
package/
├── manifest.json
├── part-0001.md
├── part-0002.md
└── ...
```

The generated package contains split Markdown files and a manifest describing the source collection.

## Pipeline

```
Markdown documentation
          |
          v
Scanner
          |
          v
Document reader
          |
          v
Inspect
          |
          v
Token counter
          |
          v
Pack splitter
          |
          v
AI-ready Markdown package
```

## Example Workflow

A typical documentation workflow:

```text
Website
  |
  v
docsync
  |
  v
Markdown files
  |
  v
docpack inspect
  |
  v
docpack pack
  |
  v
AI-ready documentation package
```

## Development

Run tests:

```bash
cargo test
```

Format:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy -- -D warnings
```

Build release:

```bash
cargo build --release
```

## License
