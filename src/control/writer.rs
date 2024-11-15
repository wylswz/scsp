use log::info;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;

use crate::context::ctx::Context;

#[derive(Deserialize, Debug)]
pub struct WriteRequest {
    #[serde(alias = "msg")]
    msg: Vec<u8>,
    channel: String,
}

#[post("/write", data = "<request>")]
pub fn write(ctx: &State<Context>, request: Json<WriteRequest>) {
    info!("publishing msg");
    let _ = ctx
        .bus
        .write()
        .map(|mut bus| bus.publish(request.channel.clone(), request.msg.clone()));
}
