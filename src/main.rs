// src/main.rs

// global import of all things Rocket
#[macro_use] extern crate rocket;

// dependencies
use rocket::fs::{FileServer, relative};

// main function
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("static")))
}