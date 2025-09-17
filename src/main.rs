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

enum ConfigMenu {
    Main,
    ApiSettings,
}

fn configure() -> Result<()> {
    println!("Configuring rup...");
    println!();

    let mut current_menu = ConfigMenu::Main;

    loop {
        // Clear screen and move to top
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush()?;

        // Load current config
        let config = Config::load().unwrap_or_else(|_| Config::default());

        // Display current config
        println!("Current Configuration:");
        println!("  API: {}", config.api.api_type);
        if let Some(time) = config.api.options.get("time") {
            println!("      Time: {}", time);
        }
        println!();

        match current_menu {
            ConfigMenu::Main => {
                println!("Configuration Menu:");
                println!("1. API Settings");
                println!("2. Save and Exit");
                print!("Enter choice (1-2): ");
                std::io::stdout().flush()?;

                let mut choice = String::new();
                std::io::stdin().read_line(&mut choice)?;
                let choice = choice.trim();

                match choice {
                    "1" => current_menu = ConfigMenu::ApiSettings,
                    "2" => break,
                    _ => {
                        println!("Invalid choice. Please try again.");
                        std::io::stdout().flush()?;
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                }
            }
            ConfigMenu::ApiSettings => {
                configure_api()?;
                current_menu = ConfigMenu::Main;
            }
        }
    }

    println!("Configuration saved!");
    Ok(())
}

fn configure_api() -> Result<()> {
    // Clear screen and move to top
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush()?;

    println!("API Settings");
    println!("------------");
    println!();
    println!("Select API type:");
    println!("1. litterbox (up to 1GB per file)");
    println!("2. temp_sh (up to 4GB per file, files expire after 3 days)");
    println!("3. uguu.se (up to 128 MiB per file, files expire after 3 hours)");
    print!("Enter choice (1-3): ");
    std::io::stdout().flush()?;

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    let api_type = match choice {
        "1" => "litterbox".to_string(),
        "2" => "temp_sh".to_string(),
        "3" => "uguu".to_string(),
        _ => {
            println!("Invalid choice. Returning to main menu.");
            std::io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            return Ok(());
        }
    };

    let mut options = std::collections::HashMap::new();

    match api_type.as_str() {
        "litterbox" => {
            // Clear screen
            print!("\x1B[2J\x1B[1;1H");
            std::io::stdout().flush()?;

            println!("API Settings - Litterbox");
            println!("------------------------");
            println!();
            println!("Select upload time:");
            println!("1. 1h");
            println!("2. 12h");
            println!("3. 24h");
            println!("4. 72h");
            print!("Enter choice (1-4): ");
            std::io::stdout().flush()?;

            let mut time_choice = String::new();
            std::io::stdin().read_line(&mut time_choice)?;
            let time = match time_choice.trim() {
                "1" => "1h",
                "2" => "12h",
                "3" => "24h",
                "4" => "72h",
                _ => {
                    println!("Invalid choice. Using default 1h.");
                    std::io::stdout().flush()?;
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    "1h"
                }
            };
            options.insert("time".to_string(), time.to_string());
        }
        "temp_sh" => {
            // No options for temp_sh
        }
        "uguu" => {
            // No options for uguu
        }
        _ => unreachable!(),
    }

    // Save API config
    let mut config = Config::load().unwrap_or_else(|_| Config::default());
    config.api = rup::config::ApiConfig { api_type, options };
    config.save()?;

    Ok(())
}


