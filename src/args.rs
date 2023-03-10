use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ChatCLIArgs {
    ///
    #[arg(short, long)]
    pub file: Option<String>,
}
