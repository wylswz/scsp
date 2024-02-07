#[macro_use] extern crate rocket;


mod data;
mod control;
mod core;
mod context;

use std::pin::Pin;

use context::ctx::Context;
use log::info;
use rocket::{fairing::AdHoc, route::BoxFuture};

#[get("/")]
fn index() -> &'static str {
    "scsp"
}

#[launch]
fn rocket() -> _ {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    rocket::build()
    .attach(AdHoc::on_response("logging", |req, resp| {
        info!("[{}]{} -> {}", req.method(), req.uri(), resp.status());
        Box::pin(async move {})
    }))
    .manage(Context::init()).mount("/", routes![
            index,
            data::writer::write,
            control::register::register
        ])
}