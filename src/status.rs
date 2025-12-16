use anyhow::Result;
use dialoguer::console::style;
use reqwest::Client;
use std::time::Instant;

pub async fn check_status(config: &crate::config::Config) -> Result<()> {
    let client = Client::new();

    // Define supported APIs
    let apis = vec![
        ("litterbox", "https://litterbox.catbox.moe/"),
        ("temp_sh", "https://temp.sh/"),
        ("uguu", "https://uguu.se/"),
        ("bashupload", "https://bashupload.com/"),
    ];

    println!("{}", style("Checking API status").bold());
    println!();

    for (name, url) in apis {
        let start = Instant::now();
        let response = client.get(url).send().await;
        let elapsed_ms = start.elapsed().as_millis();

        let (is_online, latency_display) = match response {
            Ok(resp) => {
                let ok = resp.status().is_success();
                (ok, format!("{elapsed_ms} ms"))
            }
            Err(_) => (false, "- ms".to_string()),
        };

        let status_text = if is_online {
            style("ONLINE").green().bold().to_string()
        } else {
            style("OFFLINE").red().bold().to_string()
        };

        let is_configured = name == config.api.api_type;
        let name_text = if is_configured {
            style(name).bold().to_string()
        } else {
            name.to_string()
        };
        let configured_tag = if is_configured {
            format!(" {}", style("(configured)").yellow())
        } else {
            String::new()
        };

        println!("  {}: {} ({}){}", name_text, status_text, latency_display, configured_tag);
    }

    Ok(())
}
