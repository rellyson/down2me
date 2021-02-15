#![feature(proc_macro_hygiene, decl_macro)]

mod application;
mod services;

#[macro_use] extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes![application::routes::index, application::routes::healthcheck,
    application::routes::video_download]).launch();
}
