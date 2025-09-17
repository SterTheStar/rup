use anyhow::{Result, Context};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Body, Client};
use std::fs;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncRead, BufReader, ReadBuf};
use tokio_util::io::ReaderStream;

struct ProgressReader<R> {
    inner: R,
    pb: ProgressBar,
}

impl<R: AsyncRead + Unpin> AsyncRead for ProgressReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let filled_before = buf.filled().len();
        let res = Pin::new(&mut self.inner).poll_read(cx, buf);
        if let Poll::Ready(Ok(())) = &res {
            let filled_after = buf.filled().len();
            self.pb.inc((filled_after - filled_before) as u64);
        }
        res
    }
}

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

    for path in file_paths {
        upload_file(&client, &path).await?;
    }

    Ok(())
}

async fn upload_file(client: &Client, path: &Path) -> Result<()> {
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
    let reader = BufReader::new(file);
    let progress_reader = ProgressReader { inner: reader, pb: pb.clone() };
    let stream = ReaderStream::new(progress_reader);

    let mut form = reqwest::multipart::Form::new();

    form = form.part("file", reqwest::multipart::Part::stream(Body::wrap_stream(stream)).file_name(file_name.to_string()));

    let response = client
        .post("https://temp.sh/upload")
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
