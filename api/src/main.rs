use axum::{
    routing::get,
    Router, Json, extract::Path,
};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
struct DownloadRequest {
    url: String,
    quality: Option<String>,
    output: Option<String>,
}

#[derive(Serialize)]
struct DownloadResponse {
    status: String,
    message: String,
}

async fn download_video(Path(url): Path<String>) -> Json<DownloadResponse> {
    let output = format!("{}.mp4", url.split('/').last().unwrap_or("video"));
    
    // Call the vdl binary
    let output = match Command::new("./target/debug/vdl")
        .arg(&url)
        .arg("-o")
        .arg(&output)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return Json(DownloadResponse {
                status: "error".to_string(),
                message: format!("Failed to start download: {}", e),
            });
        }
    };

    if output.status.code == Some(0) {
        Json(DownloadResponse {
            status: "success".to_string(),
            message: "Download completed".to_string(),
        })
    } else {
        Json(DownloadResponse {
            status: "error".to_string(),
            message: "Download failed".to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/download/:url", get(download_video))
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}