use std::io;
use serde_json::json;
use youtube_dl::YoutubeDl;

pub type YoutubeDlResult = Result<serde_json::Value, io::Error>;

#[tokio::main]
pub async fn download_video(video_url: String) -> YoutubeDlResult {
    let output = json!(YoutubeDl::new(video_url)
    .socket_timeout("15")
    .run()
    .unwrap());
        
    return Ok(output);
}

// let request_url = format!("https://www.youtube.com/get_video_info?video_id={}", video_url);