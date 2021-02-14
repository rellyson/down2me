use rocket::response::content;
use crate::services::youtube as ytb;

#[get("/")]
pub fn index() -> content::Json<&'static str> {
    let res = ytb::show_hello();
    content::Json(res)
}