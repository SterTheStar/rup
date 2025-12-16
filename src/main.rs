use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
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

enum ConfigMenu {
    Main,
    ApiSettings,
}

fn configure() -> Result<()> {
    println!("Configuring rup...");
    println!();

    let mut current_menu = ConfigMenu::Main;
    let theme = ColorfulTheme::default();

    loop {
        // Load current config
        let config = Config::load().unwrap_or_else(|_| Config::default());

        match current_menu {
            ConfigMenu::Main => {
                let current_api = &config.api.api_type;
                let current_time = config.api.options.get("time").cloned().unwrap_or_default();

                let menu_items = vec!["API Settings", "Save and Exit"];

                let prompt = if current_time.is_empty() {
                    format!("Current API: {current_api}")
                } else {
                    format!("Current API: {current_api} (time: {current_time})")
                };

                let selection = Select::with_theme(&theme)
                    .with_prompt(prompt)
                    .items(&menu_items)
                    .default(0)
                    .interact()?;

                match selection {
                    0 => current_menu = ConfigMenu::ApiSettings,
                    1 => break,
                    _ => unreachable!(),
                }
            }
            ConfigMenu::ApiSettings => {
                configure_api(&theme)?;
                current_menu = ConfigMenu::Main;
            }
        }
    }

    println!("Configuration saved!");
    Ok(())
}

fn configure_api(theme: &ColorfulTheme) -> Result<()> {
    let api_items = vec![
        "litterbox (up to 1GB per file)",
        "temp_sh (up to 4GB per file, files expire after 3 days)",
        "uguu.se (up to 128 MiB per file, files expire after 3 hours)",
        "bashupload (up to 50GB per file, files expire after 3 days, one-time download)",
    ];

    let api_selection = Select::with_theme(theme)
        .with_prompt("Select API type")
        .items(&api_items)
        .default(0)
        .interact()?;

    let api_type = match api_selection {
        0 => "litterbox".to_string(),
        1 => "temp_sh".to_string(),
        2 => "uguu".to_string(),
        3 => "bashupload".to_string(),
        _ => unreachable!(),
    };

    let mut options = std::collections::HashMap::new();

    if api_type == "litterbox" {
        let time_items = vec!["1h", "12h", "24h", "72h"];

        let time_selection = Select::with_theme(theme)
            .with_prompt("Select upload time for litterbox")
            .items(&time_items)
            .default(0)
            .interact()?;

        let time = match time_selection {
            0 => "1h",
            1 => "12h",
            2 => "24h",
            3 => "72h",
            _ => unreachable!(),
        };

        options.insert("time".to_string(), time.to_string());
    }

    // Save API config
    let mut config = Config::load().unwrap_or_else(|_| Config::default());
    config.api = rup::config::ApiConfig { api_type, options };
    config.save()?;

    Ok(())
}


