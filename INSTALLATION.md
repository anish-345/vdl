# 📦 VDL Installation Guide

## 🚀 Quick Install

### Option 1: Download Pre-built Binary

Choose your platform:

| Platform | Binary | Architecture |
|----------|--------|--------------|
| Linux | `vdl-linux-x64` | x86_64 |
| Linux ARM64 | `vdl-linux-aarch64` | ARM64 |
| Android | `vdl-android` | ARM64 |
| macOS | `vdl-macos` | Universal |
| Windows | `vdl-windows.exe` | x64 |
| WebAssembly | `vdl.wasm` | Browser |

### Option 2: Build from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/anish-345/vdl.git
cd vdl
cargo build --release

# Run
./target/release/vdl <URL>
```

### Option 3: Termux (Android)

```bash
pkg install rust
git clone https://github.com/anish-345/vdl.git
cd vdl
cargo build --release

# Or download pre-built Android binary
curl -L https://github.com/anish-345/vdl/releases/latest/download/vdl-android -o vdl
chmod +x vdl
./vdl <URL>
```

---

## 📋 Platform Support

✅ **Active Extractors (5+ Platforms):**

| Platform | Support Level | Features |
|----------|---------------|----------|
| **YouTube** | ✅ Active | Videos, Playlists, Live streams |
| **Instagram** | ✅ Active | Posts, Reels, Stories, IGTV |
| **Twitter/X** | ✅ Active | Videos, GIFs, Threads |
| **Vimeo** | ✅ Active | HD videos, Privacy-protected |
| **Facebook** | ✅ Active | Public videos, Groups |
| **Generic** | ✅ Fallback | Any direct video URL |

---

## 🚀 Usage Examples

```bash
# Basic download (auto-detects platform)
./vdl "https://youtube.com/watch?v=..."
./vdl "https://instagram.com/p/..."
./vdl "https://twitter.com/.../status/..."
./vdl "https://vimeo.com/..."
./vdl "https://facebook.com/..."

# Specify output file
./vdl "https://instagram.com/p/ABC123" -o my_video.mp4

# Quality selection
./vdl "https://vimeo.com/123456" -q 1080

# List available formats
./vdl "https://youtube.com/watch?v=..." --list-formats

# Silent mode (no output)
./vdl "https://facebook.com/video" -q
```

### 🎯 Platform-Specific Examples

```bash
# YouTube
./vdl "https://youtube.com/watch?v=dQw4w9WgXcQ"

# Instagram Post
./vdl "https://instagram.com/p/C5qK8vJvFPH/"

# Instagram Story
./vdl "https://instagram.com/stories/username/"

# Twitter Video
./vdl "https://twitter.com/user/status/123456789"

# Vimeo
./vdl "https://vimeo.com/123456789"

# Facebook Video
./vdl "https://facebook.com/video/posts/123456789"
```

---

## 🛠️ Command Line Options

```
USAGE:
    vdl <URL> [OPTIONS]

ARGS:
    <URL>    Video URL to download

OPTIONS:
    -o, --output <FILE>     Output filename
    -q, --quality <QUALITY> Quality (1080, 720, 480)
    -f, --format <FORMAT>   Format (mp4, webm, best, worst)
    --list-formats          List available formats
    -q, --quiet             Silent mode
    --verbose               Verbose output
    -h, --help              Show help
    -V, --version           Show version
```

---

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature-name`
3. **Add your extractor** in `src/extractors/`
4. **Test thoroughly**
5. **Submit a pull request**

### Adding a New Platform

1. Create a new file in `src/extractors/`
2. Implement the `Extractor` trait
3. Register in `src/extractors/mod.rs`
4. Add tests

```rust
// Example extractor structure
pub struct MyPlatformExtractor;

#[async_trait]
impl Extractor for MyPlatformExtractor {
    async fn extract(&self, url: &str) -> Result<VideoInfo, Error> {
        // Your extraction logic
    }
    
    fn supports(&self, url: &str) -> bool {
        url.contains("myplatform.com")
    }
}
```

---

## 📊 Version History

- **v1.0.0** - Initial release
  - Support for YouTube, Instagram, Twitter, Vimeo, Facebook
  - Cross-platform binaries
  - Format selection
  - Progress tracking

---

## 📄 License

MIT License - see LICENSE file

## 🌐 GitHub

👉 **Repository**: https://github.com/anish-345/vdl

⭐ **Star** the repo if you find it useful!

🐛 **Report issues** at https://github.com/anish-345/vdl/issues

