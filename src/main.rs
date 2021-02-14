#![feature(proc_macro_hygiene, decl_macro)]

mod infra;
mod services;

#[macro_use] extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes![infra::routes::index, infra::routes::healthcheck]).launch();
}