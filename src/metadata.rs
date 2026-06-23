use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoFormat {
    pub format_id: String,
    pub ext: String,
    pub url: String,
    pub mime_type: Option<String>,
    pub filesize: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<f32>,
    pub tbr: Option<u32>,
    pub acodec: Option<String>,
    pub vcodec: Option<String>,
    pub filename: String,
    pub title: String,
}

impl VideoFormat {
    pub fn quality_str(&self) -> String {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            format!("{}x{}", w, h)
        } else {
            "unknown".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoMetadata {
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
    pub duration: u64,
    pub uploader: String,
    pub formats: Vec<VideoFormat>,
}

impl VideoMetadata {
    pub fn list_formats(&self) {
        println!("\nAvailable formats:");
        println!("{:-<80}", "");
        println!("{:<10} {:<10} {:<15} {:<10} {:<10} {}", 
                 "FORMAT", "SIZE", "RESOLUTION", "FPS", "CODEC", "FILENAME");
        println!("{:-<80}", "");
        
        for format in &self.formats {
            let resolution = format.quality_str();
            let fps = format.fps.map(|f| format!("{:.1}", f)).unwrap_or_else(|| "N/A".to_string());
            let codec = format.vcodec.as_deref().unwrap_or("unknown");
            let size_mb = if format.filesize > 0 {
                format!("{:.1}MB", format.filesize as f64 / 1_000_000.0)
            } else {
                "N/A".to_string()
            };
            
            println!("{:<10} {:<10} {:<15} {:<10} {:<10} {}",
                     format.format_id,
                     size_mb,
                     resolution,
                     fps,
                     codec,
                     format.filename);
        }
    }
    
    pub fn select_format(&self, format_spec: &str, quality: Option<&str>) -> VideoFormat {
        match format_spec {
            "worst" => self.formats.first().cloned().unwrap_or_else(|| self.formats.last().cloned().unwrap()),
            "best" => self.formats.iter()
                .filter(|f| f.filesize > 0)
                .max_by_key(|f| {
                    f.width.unwrap_or(0) * f.height.unwrap_or(0)
                })
                .cloned()
                .unwrap_or_else(|| self.formats.last().cloned().unwrap()),
            _ => {
                if let Some(format) = self.formats.iter().find(|f| f.format_id == format_spec) {
                    return format.clone();
                }
                
                if let Some(q) = quality {
                    if let Some(format) = self.formats.iter()
                        .find(|f| {
                            f.height.map(|h| h >= q.parse().unwrap_or(0))
                                .unwrap_or(false)
                        }) {
                        return format.clone();
                    }
                }
                
                self.formats.last().cloned().unwrap()
            }
        }
    }
}