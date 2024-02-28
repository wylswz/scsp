#[macro_use]
extern crate rocket;

use std::env;

use rocket::{Config, Shutdown};
use scsp::context::ctx::Context;
use scsp::utils::flags::Parser;
use scsp::{control, data};

static FLAG_PORT: &'static str = "port";
static FLAG_PORT_ABBREV: &'static str = "p";
static DEFAULT_PORT: u16 = 6872;

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

    let mut args = env::args();
    // this is the executable
    let _ = args.next();

    let parser = Parser::new(args).expect("failed to parse args");
    let port = parser.find_either_num::<u16>(FLAG_PORT, FLAG_PORT_ABBREV).unwrap_or(DEFAULT_PORT);

    // TODO: configurable port
    rocket::build()
        .manage(ctx)
        .configure(Config {
            port,
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
