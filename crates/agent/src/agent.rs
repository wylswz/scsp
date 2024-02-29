use log::{error, info};
use scsp::client::client::{Client, DefaultClient};
use scsp::utils;
use scsp::utils::flags::Parser;
use std::env;

static FLAG_SERVER_ADDR: &str = "host";
static FLAG_SERVER_ADDR_ABBREV: &str = "h";
static DEFAULT_SERVER_ADDR: &str = "http://127.0.0.1";

static FLAG_SERVER_PORT: &str = "port";
static FLAG_SERVER_PORT_ABBREV: &str = "p";
static DEFAULT_SERVER_PORT: u16 = 6872;

pub fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let mut args = env::args();
    args.next(); // emit executable

    let parser = Parser::new(args).expect("cannot parse arguments");
    let host = parser.find_either(FLAG_SERVER_ADDR, FLAG_SERVER_ADDR_ABBREV).unwrap_or(String::from(DEFAULT_SERVER_ADDR));
    let port = parser.find_either_num(FLAG_SERVER_PORT, FLAG_SERVER_PORT_ABBREV).unwrap_or(DEFAULT_SERVER_PORT);

    let ep = format!("{}:{}", host, port);
    let client = DefaultClient::new(ep.as_str()).expect("failed to create client");

    let res = client.register("client-1", "development", |msg| {
        info!("{}", String::from_utf8(msg).unwrap());
        Ok(())
    });
    if res.is_err() {
        error!(
            "failed to register to remote server{}, {:?}",
            ep,
            res.err().unwrap()
        )
    }
}
