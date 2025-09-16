use anyhow::Result;
use clap::Parser;
use rup::cli::{Cli, Commands};
use rup::config::Config;
use rup::status;
use rup::upload;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Config) => {
            configure()?;
        }
        Some(Commands::Status) => {
            let config = Config::load()?;
            status::check_status(&config).await?;
        }
        None => {
            if cli.files.is_empty() {
                println!("No files specified. Use 'rup --help' for usage.");
                return Ok(());
            }
            let config = Config::load()?;
            upload::upload_files(cli.files, &config).await?;
        }
    }

    Ok(())
}

fn configure() -> Result<()> {
    println!("Configuring rup...");

    println!("Enter API type (currently only 'litterbox' supported): ");
    let mut api_type = String::new();
    std::io::stdin().read_line(&mut api_type)?;
    let api_type = api_type.trim().to_string();

    println!("Enter time for upload (1h, 12h, 24h, 72h): ");
    let mut time = String::new();
    std::io::stdin().read_line(&mut time)?;
    let time = time.trim().to_string();

    let config = Config {
        api: rup::config::ApiConfig { api_type, time },
    };

    config.save()?;
    println!("Config saved!");

    Ok(())
}
