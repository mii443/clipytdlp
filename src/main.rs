use clipboard_master::Master;

use crate::clipboard_handler::URLHandler;

mod yt_dlp;
mod extractor;
mod clipboard_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dest = yt_dlp::default_download_path();
    yt_dlp::download_yt_dlp(&dest).await?;
    let handler = URLHandler::new(dest);
    let master = Master::new(handler);
    master.unwrap().run().unwrap();

    Ok(())
}
