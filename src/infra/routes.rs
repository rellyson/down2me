use std::io;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::Debug;
use serde::{Serialize, Deserialize};

use crate::services::youtube as ytb;

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse {
  pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
  pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadRequest {
  pub video_url: String,
}

#[get("/healthcheck")]
pub fn healthcheck() -> Result<Json<DefaultResponse>, Debug<io::Error>>{

    let response = DefaultResponse {
        message: "Server is running healthly".to_string()
    };

    return Ok(Json(response));
}

#[get("/")]
pub fn index() -> Result<Json<DefaultResponse>, Debug<io::Error>>{

    let response = DefaultResponse {
        message: "Welcome to Down2me Media Downloader API".to_string()
    };

    return Ok(Json(response));
}

#[post("/youtube/download", format = "json", data = "<download_req>")]
pub fn download_from_youtube(download_req: Json<DownloadRequest>) -> Result<JsonValue, io::Error> {

    match ytb::download_video(download_req.video_url.to_string()) {
        Ok(response) => return Ok(JsonValue(response)),
        Err(e) => return Err(e),
    };
}
