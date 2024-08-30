<h3 align="center">
  <img src="./logo.jpeg" width="100" alt="AITools Logo"/><br/>
  <img src="https://raw.githubusercontent.com/crafter-station/website/main/public/transparent.png" height="30" width="0px"/>
  AITools
</h3>

<p align="center">
CLI-based LLM context builder for TypeScript and React projects 🚀
</p>

## About AITools

AITools is a Rust-powered command-line tool that analyzes TypeScript and React projects, extracting structured information to enhance LLM coding assistants' accuracy and context-awareness.

> While the current version focuses on TypeScript and React, we have plans to extend support to other popular languages and frameworks in the future, including Swift.

## Key Features

- 🌳 Hierarchical file structure analysis
- 🔍 TypeScript function signature extraction
- 🔀 Interactive file selection
- 🧠 LLM-optimized output format
- 📝 Neovim-based detailed view

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
