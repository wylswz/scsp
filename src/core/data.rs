use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MsgResponse {
    pub has_msg: bool,
    pub msg: Vec<u8>
}