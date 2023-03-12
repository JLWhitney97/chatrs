use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ChatCLIArgs {
    ///
    #[command(subcommand)]
    pub action: Action,
    ///
    #[arg(short, long)]
    pub file: Option<String>,
}
#[derive(Debug, Subcommand)]
pub enum Action {
    New,
    Resume,
    Quick,
}
