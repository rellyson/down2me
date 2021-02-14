#![feature(proc_macro_hygiene, decl_macro)]

mod routes;
mod services;

#[macro_use] extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes![routes::index, routes::healthcheck]).launch();
}