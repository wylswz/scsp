#[macro_use]
extern crate rocket;

mod context;
mod control;
mod core;
mod data;

use context::ctx::Context;
use rocket::Shutdown;

#[get("/")]
fn index() -> &'static str {
    "scsp"
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) {
    shutdown.notify();
}

#[launch]
fn rocket() -> _ {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let ctx = Context::init();

    rocket::build().manage(ctx).mount(
        "/",
        routes![
            shutdown,
            index,
            data::writer::write,
            control::register::register
        ],
    )
}
