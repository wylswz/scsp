use std::borrow::BorrowMut;

use rocket::serde::{Deserialize};
use rocket::serde::json::Json;
use rocket::State;

use crate::context::ctx::Context;



#[derive(Deserialize)]
#[derive(Debug)]
pub struct WriteRequest {
    #[serde(alias="msg")]
    msg: Vec<u8>,
    channel: String
}

#[post("/write", data="<request>")]
pub fn write(ctx: &State<Context>, request: Json<WriteRequest>)  {
    let _ = ctx.bus.lock().map(|mut bus| {
        bus.publish(request.channel.clone(), request.msg.clone()) 
    });
}   