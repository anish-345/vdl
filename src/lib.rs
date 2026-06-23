pub mod downloader;
pub mod extractor;
pub mod formats;

pub use downloader::Downloader;
pub use extractor::VideoInfo;
pub use formats::VideoFormat;

use std::error::Error;

pub async fn download(url: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let downloader = Downloader::new();
    let info = downloader.extract(url).await?;
    let format = info.select_format("best", None);
    
    downloader.download(&format.url, output).await?;
    
    Ok(())
}