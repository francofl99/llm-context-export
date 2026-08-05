# llme

[![CI](https://github.com/francofl99/llm-context-export/actions/workflows/ci.yml/badge.svg)](https://github.com/francofl99/llm-context-export/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`llme` is a local-first Rust CLI that exports Claude Code sessions to Markdown for review, archival, or handoff to another LLM.

> **Project status:** early-stage (`0.1.x`). The CLI and output format may change before `1.0`.

## Why llme?

Moving an active coding conversation between tools usually means manually selecting, cleaning, and explaining its context. `llme` turns a local session into a readable Markdown document with a small handoff wrapper.

- **Local-first:** reads files already stored on your machine.
- **No account or API key required:** the CLI makes no network requests.
- **Focused output:** keeps conversational text and skips thinking, tool-use, and metadata blocks.
- **Clipboard support:** copy the export without printing it to the terminal.
- **Extensible design:** provider data is normalized behind a small internal abstraction.

Currently supported provider: **Claude Code**.

## Privacy and security

Session exports can contain source code, credentials, personal data, internal URLs, or other confidential material from the original conversation. `llme` does **not** redact secrets.

Before exporting or sharing a session:

1. Review the generated Markdown.
2. Remove anything that should remain private.
3. Treat session content as untrusted when pasting it into another tool.
4. Remember that clipboard contents may be accessible to other applications on your system.

By default, `llme` only reads from `~/.claude/projects` and writes the result to standard output. With `--clipboard`, it sends the same text to the operating system's clipboard command. It does not make network requests.

See [SECURITY.md](SECURITY.md) for vulnerability reporting and the project's security model.

## How it works

Claude Code stores sessions as JSONL files below:

```text
~/.claude/projects/<project-id>/<session-id>.jsonl
```

Given a session ID, `llme`:

1. Recursively searches that directory for `<session-id>.jsonl`.
2. Parses the session one event at a time.
3. Keeps user, assistant, and system text.
4. Ignores metadata, thinking, and tool-use blocks.
5. Merges consecutive messages with the same role.
6. Renders the normalized conversation as Markdown.

Session IDs are restricted to ASCII letters, numbers, hyphens, and underscores.

## Installation

### Requirements

- A current stable [Rust toolchain](https://www.rust-lang.org/tools/install)
- Local Claude Code session files under `~/.claude/projects`
- For clipboard mode:
  - macOS: `pbcopy` (included with macOS)
  - Windows: `clip`
  - Linux: `wl-copy` or `xclip`

### Install from source

```bash
git clone https://github.com/francofl99/llm-context-export.git
cd llm-context-export
cargo install --path . --force
```

The executable is installed as `llme`, usually under `~/.cargo/bin`. Ensure that directory is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

You can also run the repository's helper script:

```bash
./install.sh
```

## Usage

Export a session to standard output:

```bash
llme export claude --session <session-id>
```

Copy it directly to the clipboard:

```bash
llme export claude --session <session-id> --clipboard
# Short form:
llme export claude --session <session-id> -c
```

Use `--help` at any level to inspect the available commands:

```bash
llme --help
llme export --help
llme export claude --help
```

### Output shape

```md
Esta es una sesión exportada de `Claude`:

# Claude Session Export

Session ID: 797df13f-41e5-4ccd-9f00-d6f6b9bee0b3

## Conversation

### User
How should I implement QR login?

---

### Claude
You can use the WhatsApp Web protocol...

Necesito que la revises, me digas si entendés el contexto, para poder continuar trabajando desde acá.
```

The wrapper is currently written in Spanish because the original workflow uses it as handoff context. Configurable templates are planned.

## Development

```bash
cargo build
cargo test --all-features
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Or run the full local verification suite:

```bash
make check
```

Run without installing:

```bash
cargo run -- export claude --session <session-id>
```

### Project structure

```text
src/
├── main.rs                 # CLI entrypoint
├── cli.rs                  # clap command definitions
├── core/                   # provider-agnostic session model
├── providers/claude.rs     # Claude discovery and JSONL parsing
└── export/                 # Markdown rendering and clipboard integration
```

## Roadmap

- File output with `--output <file.md>`
- Configurable handoff templates and languages
- Additional local LLM session providers
- Optional secret-detection or redaction assistance
- Improved Linux clipboard diagnostics

## Contributing

Issues and pull requests are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing a change and follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

Released under the [MIT License](LICENSE).
