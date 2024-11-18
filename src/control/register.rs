

use crate::context::ctx::Context;
use crate::core::data::MsgResponse;
use rocket::serde::json::Json;
use rocket::State;


/// register a websocket listener on message bus
/// block until
/// - application is shut down
/// - handler is removed from the bus
/// - message channel is deleted
#[get("/register?<client_id>&<channel>")]
#[allow(unused_variables)]
pub fn register<'r>(ctx: &State<Context>, client_id: &str, channel: &str){

}
