use anyhow::{Result, Context};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Body, Client};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_util::io::ReaderStream;

use super::ProgressReader;

pub async fn upload_files(files: Vec<String>, _config: &crate::config::Config) -> Result<()> {
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

    // Check for incompatible files for bashupload (max 50GB)
    let mut incompatible: Vec<(PathBuf, String)> = Vec::new();
    for path in &file_paths {
        let size = fs::metadata(path)?.len();
        if size > 50 * 1024 * 1024 * 1024 {
            incompatible.push((path.clone(), "File size > 50GB".to_string()));
        }
    }
    if !incompatible.is_empty() {
        println!("The following files are incompatible with bashupload API:");
        for (path, reason) in &incompatible {
            println!("- {}: {}", path.display(), reason);
        }
        if incompatible.len() == file_paths.len() {
            return Ok(());
        } else {
            println!("Proceed without these files? (y/n): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                return Ok(());
            }
            file_paths.retain(|p| !incompatible.iter().any(|(ip, _)| ip == p));
        }
    }

    for (i, path) in file_paths.iter().enumerate() {
        upload_file(&client, path, i + 1, file_paths.len()).await?;
    }

    Ok(())
}

async fn upload_file(client: &Client, path: &Path, index: usize, total: usize) -> Result<()> {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_size = fs::metadata(path)?.len();

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({percent}%, {bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Uploading {}/{}: {}", index, total, file_name));

    pb.enable_steady_tick(Duration::from_millis(80));

    let start_time = Instant::now();

    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let progress_reader = ProgressReader { inner: reader, pb: pb.clone() };
    let stream = ReaderStream::new(progress_reader);

    let mut form = reqwest::multipart::Form::new();
    form = form.part("file", reqwest::multipart::Part::stream(Body::wrap_stream(stream)).file_name(file_name.to_string()));

    let response = client
        .post("https://bashupload.com/")
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
