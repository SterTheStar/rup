use anyhow::Result;
use clap::Parser;
use rup::cli::{Cli, Commands};
use rup::config::Config;
use rup::status;
use rup::upload;
use std::io::Write;

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
    println!();

    println!("Select API type:");
    println!("1. litterbox");
    println!("2. temp_sh");
    print!("Enter choice (1-2): ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    let api_type = match choice {
        "1" => "litterbox".to_string(),
        "2" => "temp_sh".to_string(),
        _ => {
            println!("Invalid choice.");
            return Ok(());
        }
    };

    let mut options = std::collections::HashMap::new();

    match api_type.as_str() {
        "litterbox" => {
            println!();
            println!("Select upload time:");
            println!("1. 1h");
            println!("2. 12h");
            println!("3. 24h");
            println!("4. 72h");
            print!("Enter choice (1-4): ");
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut time_choice = String::new();
            std::io::stdin().read_line(&mut time_choice)?;
            let time = match time_choice.trim() {
                "1" => "1h",
                "2" => "12h",
                "3" => "24h",
                "4" => "72h",
                _ => {
                    println!("Invalid choice. Using default 1h.");
                    "1h"
                }
            };
            options.insert("time".to_string(), time.to_string());
        }
        "temp_sh" => {
            // No options for temp_sh
        }
        _ => unreachable!(),
    }

    let config = Config {
        api: rup::config::ApiConfig { api_type, options },
    };

    config.save()?;
    println!("Config saved!");

    Ok(())
}
