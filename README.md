# llme

`llme` is a local-first Rust CLI that exports Claude Code sessions to Markdown so they can be reviewed, archived, or pasted into another LLM as handoff context.

## Goals

- Work entirely with local files on disk
- No API calls
- No cloud services
- No API keys
- Be extensible to multiple LLM providers over time

Current MVP support:

- `claude`

## How it works

Claude Code stores local conversation sessions as JSONL files under:

```text
~/.claude/projects/<project-id>/<session-id>.jsonl
```

Each line is a JSON object representing an event in the session.

`llme`:

1. Searches `~/.claude/projects` recursively for the requested session ID
2. Parses the JSONL file line by line
3. Extracts conversation messages
4. Ignores internal/meta events when possible
5. Normalizes the conversation into a simple internal structure
6. Renders the session as Markdown
7. Either prints it to stdout or copies it to the clipboard

## Current output format

The exported Markdown is wrapped with extra handoff context so that if you paste it into another LLM, the receiving model understands what it is.

Example shape:

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

## Installation

### Requirements

- Rust toolchain installed
- Cargo available in your shell
- Access to local Claude Code session files under `~/.claude/projects`

### Install globally as `llme`

From the project root:

```bash
cargo install --path . --force
```

This installs the executable as:

```bash
~/.cargo/bin/llme
```

If `llme` is not found after installation, make sure this is in your shell config:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload your shell:

```bash
source ~/.zshrc
```

### Install via helper script

You can also use the provided helper:

```bash
./install.sh
```

## Usage

### Export to stdout

```bash
llme export claude --session <session-id>
```

Example:

```bash
llme export claude --session 797df13f-41e5-4ccd-9f00-d6f6b9bee0b3
```

### Export directly to clipboard

```bash
llme export claude --session <session-id> --clipboard
```

Short version:

```bash
llme export claude --session <session-id> -c
```

In clipboard mode:

- the Markdown is copied directly to the system clipboard
- the session content is not printed to stdout
- a short confirmation is printed to stderr

## Local development

### Run without installing

From the project root:

```bash
cargo run -- export claude --session <session-id>
```

Or with clipboard mode:

```bash
cargo run -- export claude --session <session-id> --clipboard
```

### Common development commands

```bash
cargo build
cargo test
cargo install --path . --force
```

## Makefile

The project includes a `Makefile` for common tasks.

Available commands:

```bash
make help
make build
make test
make install
make run ARGS="export claude --session <session-id>"
```

Examples:

```bash
make test
make install
make run ARGS="export claude --session 797df13f-41e5-4ccd-9f00-d6f6b9bee0b3 --clipboard"
```

## Project structure

```text
src/
  main.rs
  cli.rs
  providers/
    mod.rs
    claude.rs
  core/
    session.rs
    message.rs
  export/
    markdown.rs
    clipboard.rs
```

### Key modules

- `src/main.rs` — CLI entrypoint and top-level command flow
- `src/cli.rs` — `clap` command definitions
- `src/providers/mod.rs` — provider abstraction
- `src/providers/claude.rs` — Claude session discovery and JSONL parsing
- `src/core/session.rs` — normalized session model and message merging
- `src/core/message.rs` — message roles and message type
- `src/export/markdown.rs` — Markdown rendering and handoff wrapper
- `src/export/clipboard.rs` — clipboard integration

## Internal model

The project normalizes provider data into a provider-agnostic structure:

```rust
struct Session {
    id: String,
    messages: Vec<Message>,
}

struct Message {
    role: Role,
    content: String,
}

enum Role {
    User,
    Assistant,
    System,
}
```

Providers implement a common abstraction:

```rust
trait Provider {
    fn load_session(session_id: &str) -> Result<Session>;
}
```

This makes it easier to add providers later such as:

- OpenAI
- Cursor
- Ollama
- others with local session storage

## Claude provider behavior

The Claude provider currently does the following:

- recursively searches inside `~/.claude/projects`
- looks for `<session-id>.jsonl`
- reads the file line by line
- parses each line as JSON
- keeps only meaningful conversation events
- extracts text from user/assistant/system messages
- ignores metadata/internal events like progress and attachments
- ignores assistant blocks such as `thinking` or `tool_use`
- merges consecutive same-role messages to produce cleaner output

This merge behavior matters because Claude often stores assistant output as multiple streamed events.

## Notes about clipboard support

Clipboard export is implemented locally.

Current behavior by platform:

- macOS: uses `pbcopy`
- Windows: uses `clip`
- Linux/Unix: tries `wl-copy`, then `xclip`

For the current local environment, macOS is the main verified path.

## Example workflow

A common workflow is:

1. Work with Claude Code locally
2. Export a session with `llme`
3. Copy it to the clipboard
4. Paste it into another LLM
5. Ask that LLM to continue from the exported context

Example:

```bash
llme export claude --session 797df13f-41e5-4ccd-9f00-d6f6b9bee0b3 --clipboard
```

Then paste into another LLM conversation.

## Updating after code changes

If you modify the code:

- `cargo build` only rebuilds the project locally
- it does **not** update the globally installed `llme`

To update the global command, run:

```bash
cargo install --path . --force
```

Or simply:

```bash
./install.sh
```

Or:

```bash
make install
```

## Future improvements

Potential next steps:

- add file output support such as `--output <file.md>`
- support more providers
- add configurable handoff templates
- improve Linux clipboard detection and messaging
- add `fmt`, `clippy`, and `clean` Makefile targets

## License

At the moment, no license file has been added yet.
