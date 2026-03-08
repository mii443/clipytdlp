use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler};
use std::path::PathBuf;
use crate::{extractor, notification, yt_dlp};

pub struct URLHandler {
    yt_dlp_path: PathBuf,
    clipboard_ctx: ClipboardContext,
}

impl URLHandler {
    pub fn new(yt_dlp_path: PathBuf) -> Self {
        Self {
            yt_dlp_path,
            clipboard_ctx: ClipboardContext::new().unwrap(),
        }
    }
}

impl ClipboardHandler for URLHandler {
    fn on_clipboard_change(&mut self) -> clipboard_master::CallbackResult {
        if let Ok(contents) = self.clipboard_ctx.get_contents() {
            if extractor::is_valid_youtube_url(&contents) {
                println!("Valid YouTube URL: {}", contents);
                let url = extractor::extract_video_id(&contents).unwrap();
                let download_url = yt_dlp::get_download_url(&url, &self.yt_dlp_path).unwrap();
                println!("Download URL: {}", download_url);
                self.clipboard_ctx.set_contents(download_url.clone()).unwrap();
                notification::notify_success(&contents, &download_url);
            }
        }

        CallbackResult::Next
    }
}