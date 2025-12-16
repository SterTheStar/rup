use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rup",
    version,
    about = "Upload files quickly to litterbox, temp.sh, uguu.se, or bashupload.",
    long_about = "rup is a simple CLI for uploading files to temporary file hosting services.\n\
\n\
By default it uploads files using your configured API.\n\
\n\
Examples:\n  rup file.txt\n  rup *.png\n  rup * temp_sh\n\nUse `rup config` to choose the API and options."
)]
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
