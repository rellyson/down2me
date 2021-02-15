use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;

use youtube_dl::YoutubeDl;

#[derive(Serialize, Deserialize, Debug)]
struct YoutubeDlData {
    SingleVideo: VideoInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct VideoInfo {
    thumbnail: String,
    title: String,
    formats: Vec<Format>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Format {
    url: String,
    ext: String,
    format: String,
    filesize: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
}

#[tokio::main]
pub async fn download_video(video_url: String) -> Result<serde_json::Value, io::Error> {
    let output = json!(YoutubeDl::new(video_url)
        .socket_timeout("15")
        .run()
        .unwrap());

    let data: YoutubeDlData = serde_json::from_str(&output.to_string())?;
    return Ok(json!(data.SingleVideo));
}
