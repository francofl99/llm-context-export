use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::{
    core::{Role, Session},
    providers::Provider,
};

pub struct ClaudeProvider;

impl Provider for ClaudeProvider {
    fn load_session(session_id: &str) -> Result<Session> {
        validate_session_id(session_id)?;
        let path = find_session_file(session_id)?;
        parse_session_file(session_id, &path)
    }
}

fn validate_session_id(session_id: &str) -> Result<()> {
    const MAX_SESSION_ID_LENGTH: usize = 128;

    if session_id.is_empty() || session_id.len() > MAX_SESSION_ID_LENGTH {
        bail!("session ID must contain between 1 and {MAX_SESSION_ID_LENGTH} characters");
    }

    if !session_id
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        bail!("session ID may contain only ASCII letters, numbers, hyphens, and underscores");
    }

    Ok(())
}

fn find_session_file(session_id: &str) -> Result<PathBuf> {
    let root = claude_projects_dir()?;
    let expected_name = format!("{session_id}.jsonl");

    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        if entry.file_name() == OsStr::new(&expected_name) {
            return Ok(entry.into_path());
        }
    }

    bail!(
        "could not find Claude session {session_id} under {}",
        root.display()
    )
}

fn claude_projects_dir() -> Result<PathBuf> {
    let home = env::var_os("HOME").context("HOME environment variable is not set")?;
    Ok(PathBuf::from(home).join(".claude").join("projects"))
}

fn parse_session_file(session_id: &str, path: &Path) -> Result<Session> {
    let file = File::open(path)
        .with_context(|| format!("failed to open Claude session file {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut session = Session::new(session_id);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!(
                "failed to read line {} from {}",
                line_number + 1,
                path.display()
            )
        })?;

        if line.trim().is_empty() {
            continue;
        }

        let event: ClaudeEvent = serde_json::from_str(&line).with_context(|| {
            format!(
                "failed to parse JSON on line {} from {}",
                line_number + 1,
                path.display()
            )
        })?;

        if let Some((role, content)) = event.into_message()? {
            session.push_message(role, content);
        }
    }

    Ok(session)
}

#[derive(Debug, Deserialize)]
struct ClaudeEvent {
    #[serde(rename = "type")]
    event_type: String,
    message: Option<ClaudeMessage>,
}

impl ClaudeEvent {
    fn into_message(self) -> Result<Option<(Role, String)>> {
        let role = match self.event_type.as_str() {
            "user" => Role::User,
            "assistant" => Role::Assistant,
            "system" => Role::System,
            _ => return Ok(None),
        };

        let Some(message) = self.message else {
            // Many Claude JSONL events are metadata-only (progress, attachments, timing).
            // We ignore them for the MVP unless they contain a real textual message.
            return Ok(None);
        };

        let content = extract_text_from_content(message.content)?;
        if content.trim().is_empty() {
            return Ok(None);
        }

        Ok(Some((role, content)))
    }
}

#[derive(Debug, Deserialize)]
struct ClaudeMessage {
    content: ClaudeContent,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ClaudeContent {
    Text(String),
    Blocks(Vec<ClaudeContentBlock>),
}

#[derive(Debug, Deserialize)]
struct ClaudeContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

fn extract_text_from_content(content: ClaudeContent) -> Result<String> {
    match content {
        ClaudeContent::Text(text) => Ok(text),
        ClaudeContent::Blocks(blocks) => {
            let text_blocks = blocks
                .into_iter()
                .filter_map(|block| match block.block_type.as_str() {
                    // Keep only actual conversational text. We deliberately skip
                    // thinking/tool blocks so the export stays human-readable.
                    "text" => block.text,
                    _ => None,
                })
                .collect::<Vec<_>>();

            Ok(text_blocks.join("\n\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_plain_string_content() {
        let content = ClaudeContent::Text("hello".to_string());
        assert_eq!(extract_text_from_content(content).unwrap(), "hello");
    }

    #[test]
    fn extracts_only_text_blocks() {
        let content = ClaudeContent::Blocks(vec![
            ClaudeContentBlock {
                block_type: "thinking".to_string(),
                text: None,
            },
            ClaudeContentBlock {
                block_type: "text".to_string(),
                text: Some("first".to_string()),
            },
            ClaudeContentBlock {
                block_type: "tool_use".to_string(),
                text: Some("ignored".to_string()),
            },
            ClaudeContentBlock {
                block_type: "text".to_string(),
                text: Some("second".to_string()),
            },
        ]);

        assert_eq!(
            extract_text_from_content(content).unwrap(),
            "first\n\nsecond"
        );
    }

    #[test]
    fn accepts_expected_session_ids() {
        assert!(validate_session_id("797df13f-41e5-4ccd-9f00-d6f6b9bee0b3").is_ok());
        assert!(validate_session_id("agent_session-123").is_ok());
    }

    #[test]
    fn rejects_session_ids_with_path_or_control_characters() {
        assert!(validate_session_id("").is_err());
        assert!(validate_session_id("../session").is_err());
        assert!(validate_session_id("session/other").is_err());
        assert!(validate_session_id("session\nheader").is_err());
    }
}
