
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{context::ctx::Context, core::pubsub::ChannelSummary};



#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    channels: Vec<ChannelSummary>
}

#[get("/info")]
pub fn info(state: &State<Context>) -> Json<Info>{
    let info = state.bus.read().map(|bus| {
        let handlers = bus.list_handler();
        Info{
            channels: handlers.channels
        }
    }).unwrap();
    return Json::from(info);
}
