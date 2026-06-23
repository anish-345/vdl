# VDL - Video Downloader Library

A modular, cross-platform video downloader built with Rust.

## 🚀 Features

- **Memory Safe** - Built with Rust for zero-cost abstractions
- **Cross-Platform** - Linux, macOS, Windows, Android (Termux)
- **Extensible Architecture** - Plugin system for new platforms
- **Multiple Protocols** - MP4, M3U8 (HLS), MPD (DASH)
- **Progress Tracking** - Real-time download progress
- **Format Selection** - Automatic or manual quality selection

## 📦 Installation

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version
cargo --version
```

### Build
```bash
cd vdl
cargo build --release
```

### Run
```bash
./target/release/vdl <URL>
```

## 🛠️ For Developers

### Project Structure
```
vdl/
├── src/
│   ├── lib.rs          # Library exports
│   ├── main.rs         # CLI entry point
│   ├── downloader.rs   # HTTP download logic
│   ├── extractor.rs    # Base extraction trait
│   └── formats.rs      # Format handling
├── examples/           # Usage examples
├── Cargo.toml          # Dependencies
└── README.md
```

### Using as a Library

Add to your `Cargo.toml`:
```toml
[dependencies]
vdl = { path = "path/to/vdl" }
```

Basic usage:
```rust
use vdl::{Downloader, VideoInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = Downloader::new();
    
    // Extract video info
    let info = downloader.extract("https://example.com/video").await?;
    
    // Download best quality
    downloader.download(&info.formats.best, "video.mp4").await?;
    
    Ok(())
}
```

### Creating a New Extractor

Implement the `Extractor` trait:

```rust
use async_trait::async_trait;
use crate::{Extractor, VideoInfo, VideoFormat};

pub struct MyPlatformExtractor;

#[async_trait]
impl Extractor for MyPlatformExtractor {
    async fn extract(&self, url: &str) -> Result<VideoInfo, Error> {
        // Your extraction logic here
        Ok(VideoInfo {
            id: "video_id".to_string(),
            title: "Video Title".to_string(),
            formats: vec![VideoFormat {
                id: "best".to_string(),
                url: "https://cdn.com/video.mp4".to_string(),
                quality: 1080,
                ..Default::default()
            }],
        })
    }
    
    fn supports(&self, url: &str) -> bool {
        url.contains("myplatform.com")
    }
}
```

Register in `extractor.rs`:
```rust
pub fn register_extractors() -> Vec<Box<dyn Extractor>> {
    vec![
        Box::new(YouTubeExtractor),
        Box::new(MyPlatformExtractor),
        // Add more...
    ]
}
```

## 📖 CLI Usage

```bash
# Basic download
vdl <URL>

# Specify output
vdl <URL> -o output.mp4

# Quality selection
vdl <URL> -q 720

# List formats
vdl <URL> --formats

# Silent mode
vdl <URL> -q
```

## 🌐 Web API

VDL can be deployed as a web service for browser-based video downloading:

### API Server
```bash
cd api
cargo build
cargo run
```

**Endpoints:**
```
GET /download/:url    # Download video by URL
```

### Web Integration
```javascript
// Frontend JavaScript
async function downloadVideo(url) {
    const response = await fetch(`http://localhost:8080/download/${encodeURIComponent(url)}`);
    const result = await response.json();
    console.log(result);
}
```

## 🧬 Architecture

```
┌─────────────────────────────────────────┐
│                 CLI/Application         │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│           Downloader Interface          │
│  - Extract video info                   │
│  - Select optimal format                │
│  - Download with progress               │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│            Extractor Router             │
│  - URL → Platform detection             │
│  - Route to appropriate extractor       │
└──────────────────┬──────────────────────┘
                   │
     ┌─────────────┴─────────────┐
     │                           │
┌────▼────┐                 ┌────▼────┐
│YouTube  │                 │Generic  │
│Extractor│                 │Extractor│
└─────────┘                 └─────────┘
```

## 🎯 Roadmap

- [ ] Plugin system for external extractors
- [ ] Concurrent download support
- [ ] Post-processing hooks (thumbnail, subtitles)
- [ ] Metadata embedding (ID3, YUV)
- [ ] Queue management

## 📜 License

MIT - See LICENSE file

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Add tests
4. Submit pull request