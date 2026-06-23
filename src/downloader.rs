use indicatif::ProgressBar;
use log::debug;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use futures::StreamExt;

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn download(
        &self,
        url: &str,
        path: &Path,
        show_progress: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Downloading from: {}", url);
        debug!("Saving to: {:?}", path);

        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        let total_size = response.content_length().unwrap_or(0) as usize;
        
        let mut file = File::create(path)?;
        let downloaded = AtomicUsize::new(0);
        let mut stream = response.bytes_stream();

        let pb = if show_progress && total_size > 0 {
            let pb = ProgressBar::new(total_size as u64);
            Some(pb)
        } else {
            None
        };

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
            let chunk_size = chunk.len();
            
            file.write_all(&chunk)?;
            
            downloaded.fetch_add(chunk_size, Ordering::SeqCst);
            
            if let Some(ref pb) = pb {
                pb.set_position(downloaded.load(Ordering::SeqCst) as u64);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        Ok(())
    }
}