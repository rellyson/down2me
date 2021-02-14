use std::io;
use rocket_contrib::json::Json;
use rocket::response::Debug;
use serde::{Serialize, Deserialize};

use crate::services::youtube as ytb;

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse {
  message: String,
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
