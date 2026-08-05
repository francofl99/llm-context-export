# Security Policy

## Supported versions

`llme` is currently pre-1.0. Security fixes are applied to the latest code on the default branch and released in the newest available version.

## Reporting a vulnerability

Please do not disclose suspected vulnerabilities in a public issue.

Use GitHub's **Report a vulnerability** option in the repository's Security tab when available. If private vulnerability reporting is unavailable, contact the repository owner privately through their GitHub profile before sharing technical details.

Include:

- the affected version or commit;
- steps to reproduce the issue;
- the expected impact;
- any suggested mitigation.

Do not include real credentials, unredacted session exports, or third-party private data in the report.

## Security model

`llme` is a local CLI. It:

- reads Claude Code JSONL files below `~/.claude/projects`;
- does not require credentials or make network requests;
- writes exported content to standard output or a local clipboard command;
- does not redact or classify secrets in session content.

Users are responsible for reviewing exports before storing, sharing, or submitting them to another service. Session content and clipboard contents should be treated as sensitive and potentially untrusted.

The project does not consider accidental sharing of secrets already present in a session to be a vulnerability in the CLI, unless `llme` bypasses an explicit safeguard or reads data outside its documented scope.
