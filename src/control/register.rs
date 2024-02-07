use std::borrow::{Borrow, BorrowMut};
use std::sync::Arc;

use log::info;
use rocket::request::FromRequest;
use rocket::{serde::json::Json, State};
use serde::Deserialize;
use crate::context::ctx::Context;
use crate::core::errs;
use crate::core::pubsub::MsgHandler;


#[derive(Deserialize)]
pub struct RegisterRequest<'r> {
    channel: Box<str>,
    #[serde(alias="clientId")]
    client_id: &'r str,
}


struct LoggingHandler {
    channel: Box<str>
}
impl LoggingHandler {
    fn new() -> Self {
        LoggingHandler {
            channel: "default".into()
        }
    }

    fn on_channel(channel: Box<str>) -> Self {
        LoggingHandler {
            channel: channel
        }
    }
}

impl MsgHandler for LoggingHandler {
    fn handle(&self, msg: Vec<u8>) -> Result<(), crate::core::errs::SCSPErr> {
        info!("from logging handler: {:?}", String::from_utf8(msg));
        Ok(())
    }

    fn identity(&self) -> &str {
        "system"
    }

    fn channel(&self) -> &str {
        &self.channel
    }
}


#[post("/register", data="<request>")]
pub fn register<'r>(ctx: &State<Context>, request: Json<RegisterRequest>) -> Result<(), errs::SCSPErr<'r>> {
    let h = LoggingHandler::on_channel(request.channel.clone());
    _ = ctx.bus.lock().and_then(|mut b| {
        b.register_handler(Arc::new(h));
        Ok(())
    });
    Ok(())
}


struct Handler<'a>  {
    identifier: &'a str
}
