use anyhow::{Result, Context};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

pub async fn upload_files(files: Vec<String>, config: &crate::config::Config) -> Result<()> {
    let client = Client::new();
    let mut file_paths = Vec::new();

    for file in files {
        if file == "*" {
            let entries = fs::read_dir(".").context("Failed to read current directory")?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    file_paths.push(path);
                }
            }
        } else {
            let path = Path::new(&file);
            if path.exists() && path.is_file() {
                file_paths.push(path.to_path_buf());
            } else {
                eprintln!("File not found: {}", file);
            }
        }
    }

    for path in file_paths {
        upload_file(&client, &path, config).await?;
    }

    Ok(())
}

async fn upload_file(client: &Client, path: &Path, config: &crate::config::Config) -> Result<()> {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_size = fs::metadata(path)?.len();

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Uploading {}", file_name));

    let start_time = Instant::now();

    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);

    let mut form = reqwest::multipart::Form::new()
        .text("reqtype", "fileupload")
        .text("time", config.api.time.clone());

    // For progress, we need to read the file and add to multipart
    // But multipart in reqwest doesn't support streaming with progress easily.
    // So, read the entire file into memory for simplicity (assuming files aren't too large).
    // For large files, we'd need a custom stream.

    let mut data = Vec::new();
    reader.read_to_end(&mut data).await?;
    form = form.part("fileToUpload", reqwest::multipart::Part::bytes(data).file_name(file_name.to_string()));

    let response = client
        .post("https://litterbox.catbox.moe/resources/internals/api.php")
        .multipart(form)
        .send()
        .await
        .context("Failed to send request")?;

    if response.status().is_success() {
        let url = response.text().await?;
        pb.finish_with_message(format!("Uploaded {} to {}", file_name, url.trim()));

        let elapsed = start_time.elapsed();
        let speed_mbps = (file_size as f64 / 1_000_000.0) / elapsed.as_secs_f64() * 8.0; // Mbps
        println!("Upload speed: {:.2} Mbps", speed_mbps);
    } else {
        pb.finish_with_message(format!("Failed to upload {}", file_name));
        return Err(anyhow::anyhow!("Upload failed with status: {}", response.status()));
    }

    Ok(())
}
