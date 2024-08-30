# AITools: TypeScript/React Analyzer to Enhance LLM Coding Assistants

AITools is a Rust-based CLI tool that analyzes TypeScript and React projects, extracting structured information to significantly improve the accuracy of LLMs responses. 

## Key Features

- Hierarchical file structure analysis
- TypeScript function signature extraction
- Interactive file selection
- LLM-optimized output format
- Neovim-based detailed view

## Quick Start

1. Build:

```bash
git clone https://github.com/yourusername/aitools.git
cd aitools
cargo build --release
```

2. Run:

```bash
./target/release/aitools --path /path/to/your/project
```

## Configuration

Customize `config.json` for ignore patterns and output preferences.

## Primary Goal

Bridge the gap between complex TypeScript/React projects and LLM coding assistants by providing rich, contextual data. This enables LLMs to generate more accurate, context-aware, and project-specific code suggestions.

## Future Enhancements

- Multi-language support (Swift)
