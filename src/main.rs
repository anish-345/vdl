use clap::Parser;
use log::{debug, info};
use std::path::PathBuf;

mod extractors;
mod downloader;
mod metadata;

use downloader::Downloader;
use extractors::Platform;
use metadata::VideoMetadata;

/// A Rust-based video downloader supporting multiple platforms
/// Inspired by yt-dlp with cross-platform support
#[derive(Parser, Debug)]
#[command(name = "vdl")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the video to download
    #[arg(required = true)]
    url: String,

    /// Output filename
    #[arg(short, long)]
    output: Option<String>,

    /// Format selection (e.g., "mp4", "webm", "best", "worst")
    #[arg(short = 'f', long, default_value = "best")]
    format: String,

    /// Quality selection (e.g., "1080", "720", "480")
    #[arg(short = 'Q', long)]
    quality: Option<String>,

    /// Work silently
    #[arg(short, long)]
    quiet: bool,

    /// Verbose output
    #[arg(long)]
    verbose: bool,

    /// List available formats
    #[arg(long)]
    list_formats: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logger
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
            .init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .init();
    }

    // Parse URL to determine platform
    let url = url::Url::parse(&args.url)?;
    let platform = extractors::detect_platform(&url);
    
    debug!("Processing URL: {} on platform: {:?}", args.url, platform);

    // Extract video metadata
    let metadata: VideoMetadata = match platform {
        Platform::YouTube => {
            info!("Extracting YouTube video metadata...");
            extractors::youtube::extract(&args.url).await?
        }
        Platform::Vimeo => {
            info!("Extracting Vimeo video metadata...");
            extractors::vimeo::extract(&args.url).await?
        }
        Platform::Twitter => {
            info!("Extracting Twitter video metadata...");
            extractors::twitter::extract(&args.url).await?
        }
        Platform::Instagram => {
            info!("Extracting Instagram video metadata...");
            extractors::instagram::extract(&args.url).await?
        }
        Platform::Facebook => {
            info!("Extracting Facebook video metadata...");
            extractors::facebook::extract(&args.url).await?
        }
        Platform::Unknown => {
            return Err("Unsupported platform or URL".into());
        }
    };

    // List formats if requested
    if args.list_formats {
        metadata.list_formats();
        return Ok(());
    }

    // Select format based on user preferences
    let format = metadata.select_format(&args.format, args.quality.as_deref());
    
    debug!("Selected format: {:?}", format);

    // Determine output path
    let output_path = if let Some(ref output) = args.output {
        PathBuf::from(output)
    } else {
        PathBuf::from(format.filename.clone())
    };

    // Download video
    info!("Downloading: {}", metadata.title);
    
    let downloader = Downloader::new();
    downloader.download(&format.url, &output_path, !args.quiet).await?;
    
    info!("✓ Download completed: {}", output_path.display());
    
    Ok(())
}