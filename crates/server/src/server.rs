#[macro_use]
extern crate rocket;

use rocket::{Config, Shutdown};
use scsp::context::ctx::Context;
use scsp::{control, data};

#[get("/")]
fn index() -> &'static str {
    "scsp"
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) {
    shutdown.notify();
}

#[launch]
pub fn rocket() -> _ {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let ctx = Context::init();

    // TODO: configurable port
    rocket::build()
        .manage(ctx)
        .configure(Config {
            port: 6872,
            ..Default::default()
        })
        .mount(
            "/",
            routes![
                shutdown,
                index,
                data::writer::write,
                control::register::register
            ],
        )
}
