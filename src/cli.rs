use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "llme")]
#[command(about = "Export local LLM coding sessions to Markdown")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Export a session to Markdown.
    Export {
        #[command(subcommand)]
        provider: ExportProvider,
    },
}

#[derive(Debug, Subcommand)]
pub enum ExportProvider {
    /// Export a Claude Code session stored under ~/.claude/projects.
    Claude {
        /// Claude Code session ID (for example: 797df13f-41e5-4ccd-9f00-d6f6b9bee0b3)
        #[arg(long)]
        session: String,

        /// Copy the Markdown export to the system clipboard instead of printing it.
        #[arg(long, short = 'c')]
        clipboard: bool,
    },
}
