use log::{error, info};
use scsp::client::client::{Client, DefaultClient};
use scsp::utils;
use std::env;

pub fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let mut args = env::args();
    args.next(); // emit executable
    let ep = args.next().unwrap_or(String::from("http://127.0.0.1:6872"));
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
