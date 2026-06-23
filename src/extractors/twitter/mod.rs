use crate::metadata::{VideoMetadata, VideoFormat};

pub async fn extract(_url: &str) -> Result<VideoMetadata, Box<dyn std::error::Error>> {
    Ok(VideoMetadata {
        id: "twitter-sample".to_string(),
        title: "Twitter Video".to_string(),
        description: String::new(),
        thumbnail: "https://pbs.twimg.com/sample.jpg".to_string(),
        duration: 0,
        uploader: String::new(),
        formats: vec![VideoFormat {
            format_id: "high".to_string(),
            ext: "mp4".to_string(),
            url: "https://video.twimg.com/sample.mp4".to_string(),
            mime_type: Some("video/mp4".to_string()),
            filesize: 10_000_000,
            width: Some(1280),
            height: Some(720),
            fps: Some(30.0),
            tbr: Some(1000),
            acodec: Some("aac".to_string()),
            vcodec: Some("h264".to_string()),
            filename: "twitter-video.mp4".to_string(),
            title: "Twitter Video".to_string(),
        }],
    })
}