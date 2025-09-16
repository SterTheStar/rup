use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rup")]
#[command(about = "A CLI tool for uploading files to various APIs")]
pub struct Cli {
    /// Files to upload, or '*' for all files in current directory
    pub files: Vec<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure the app settings
    Config,
    /// Show the status of each API
    Status,
}
