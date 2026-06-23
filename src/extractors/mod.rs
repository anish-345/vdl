pub mod youtube;
pub mod vimeo;
pub mod twitter;
pub mod instagram;
pub mod facebook;

use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    YouTube,
    Vimeo,
    Twitter,
    Instagram,
    Facebook,
    Unknown,
}

pub fn detect_platform(url: &Url) -> Platform {
    let host = url.host_str().unwrap_or("").to_lowercase();
    
    if host.contains("youtube") || host.contains("youtu.be") {
        Platform::YouTube
    } else if host.contains("vimeo") {
        Platform::Vimeo
    } else if host.contains("twitter") || host.contains("t.co") {
        Platform::Twitter
    } else if host.contains("instagram") {
        Platform::Instagram
    } else if host.contains("facebook") {
        Platform::Facebook
    } else {
        Platform::Unknown
    }
}