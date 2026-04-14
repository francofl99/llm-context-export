use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

pub fn copy_text(text: &str) -> Result<()> {
    let mut child = clipboard_command()
        .stdin(Stdio::piped())
        .spawn()
        .context("failed to start clipboard command")?;

    let mut stdin = child
        .stdin
        .take()
        .context("failed to open clipboard command stdin")?;
    stdin
        .write_all(text.as_bytes())
        .context("failed to write export to clipboard")?;
    drop(stdin);

    let status = child.wait().context("failed to wait for clipboard command")?;
    if !status.success() {
        bail!("clipboard command exited with status {status}");
    }

    Ok(())
}

fn clipboard_command() -> Command {
    #[cfg(target_os = "macos")]
    {
        Command::new("pbcopy")
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("clip")
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // Linux support is best-effort for future reuse. We prefer wl-copy when
        // available, then fall back to xclip.
        if command_exists("wl-copy") {
            return Command::new("wl-copy");
        }

        if command_exists("xclip") {
            let mut command = Command::new("xclip");
            command.args(["-selection", "clipboard"]);
            return command;
        }

        Command::new("false")
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {command} >/dev/null 2>&1"))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
