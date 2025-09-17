use anyhow::Result;
use reqwest::Client;

pub async fn check_status(config: &crate::config::Config) -> Result<()> {
    let client = Client::new();

    // Define supported APIs
    let apis = vec![
        ("litterbox", "https://litterbox.catbox.moe/"),
        ("temp_sh", "https://temp.sh/"),
        ("uguu", "https://uguu.se/"),
    ];

    println!("Checking API status:");

    for (name, url) in apis {
        let response = client.get(url).send().await;
        let status = if let Ok(resp) = response {
            if resp.status().is_success() {
                "Online"
            } else {
                "Offline"
            }
        } else {
            "Offline"
        };
        let configured = if name == config.api.api_type { " [configured]" } else { "" };
        println!("- {}: {}{}", name, status, configured);
    }

    Ok(())
}
