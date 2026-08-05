mod cli;
mod core;
mod export;
mod providers;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ExportProvider};
use export::{clipboard, markdown};
use providers::Provider;
use providers::claude::ClaudeProvider;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Export { provider } => {
            let (title, session, to_clipboard) = match provider {
                ExportProvider::Claude { session, clipboard } => {
                    ("Claude", ClaudeProvider::load_session(&session)?, clipboard)
                }
            };

            let markdown = markdown::render(&session, title);

            if to_clipboard {
                clipboard::copy_text(&markdown)?;
                eprintln!(
                    "Copied Markdown export for session {} to the clipboard.",
                    session.id
                );
            } else {
                println!("{markdown}");
            }
        }
    }

    Ok(())
}
