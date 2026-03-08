use std::{path::{Path, PathBuf}, process::Command};

use futures_util::StreamExt;
use tokio::{fs, io::AsyncWriteExt};

const YT_DLP_URL: &str =
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";

#[derive(Debug, thiserror::Error)]
pub enum YtDlpError {
    #[error("failed to run yt-dlp")]
    CommandExec(#[source] std::io::Error),

    #[error("failed to decode yt-dlp output as UTF-8")]
    InvalidUtf8(#[source] std::string::FromUtf8Error),

    #[error("download failed (HTTP {0})")]
    HttpStatus(reqwest::StatusCode),

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("failed to create output directory")]
    CreateDir(#[source] std::io::Error),

    #[error("failed to create file")]
    CreateFile(#[source] std::io::Error),

    #[error("failed to write to file")]
    WriteFile(#[source] std::io::Error),

    #[error("failed to flush file")]
    FlushFile(#[source] std::io::Error),
}

pub fn get_download_url(url: &str, yt_dlp_path: &Path) -> Result<String, YtDlpError> {
    let output = Command::new(yt_dlp_path)
        .args(["--quiet", "--get-url"])
        .args(["-f", "best[height<=1080][acodec!=none]"])
        .args(["-S", "res:1080"])
        .arg(url)
        .output()
        .map_err(YtDlpError::CommandExec)?;
    let stdout = String::from_utf8(output.stdout).map_err(YtDlpError::InvalidUtf8)?;
    Ok(stdout.trim().to_string())
}

pub fn default_download_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("yt-dlp.exe")
}

pub async fn download_yt_dlp(dest: &Path) -> Result<(), YtDlpError> {
    println!("Downloading yt-dlp: {YT_DLP_URL}");

    let response = reqwest::get(YT_DLP_URL).await?;

    let status = response.status();
    if !status.is_success() {
        return Err(YtDlpError::HttpStatus(status));
    }

    let total_size = response.content_length();

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(YtDlpError::CreateDir)?;
    }

    let mut file = fs::File::create(dest)
        .await
        .map_err(YtDlpError::CreateFile)?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)
            .await
            .map_err(YtDlpError::WriteFile)?;

        downloaded += chunk.len() as u64;
        if let Some(total) = total_size {
            let pct = (downloaded as f64 / total as f64 * 100.0) as u32;
            print!("\r  Progress: {downloaded} / {total} bytes ({pct}%)");
        } else {
            print!("\r  Downloaded: {downloaded} bytes");
        }
    }

    file.flush().await.map_err(YtDlpError::FlushFile)?;
    println!("\n  Saved to: {}", dest.display());
    Ok(())
}
