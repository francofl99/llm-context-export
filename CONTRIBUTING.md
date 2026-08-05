# Contributing

Thanks for helping improve `llme`.

## Before opening an issue

- Search existing issues to avoid duplicates.
- Do not include real session exports, credentials, private source code, or personal data.
- For security vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.

## Development setup

1. Install a current stable Rust toolchain.
2. Fork and clone the repository.
3. Create a focused branch.
4. Run the verification suite:

   ```bash
   make check
   ```

You can run the CLI directly during development:

```bash
cargo run -- export claude --session <session-id>
```

Use synthetic or thoroughly redacted JSONL fixtures when adding tests. Never commit files from `~/.claude/projects`.

## Pull requests

- Keep each pull request focused on one concern.
- Add or update tests for behavior changes.
- Update documentation when commands, output, or platform support changes.
- Ensure formatting, Clippy, and tests pass.
- Explain any privacy or compatibility implications in the pull request description.

By contributing, you agree that your contribution will be licensed under the MIT License.
