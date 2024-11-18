
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{context::ctx::Context};


#[get("/info")]
pub fn info(state: &State<Context>) {

}
