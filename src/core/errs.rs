use rocket::response::Responder;
use url::ParseError;
use websocket::WebSocketError;

#[derive(Responder, Debug)]
pub struct SCSPErr {
    msg: String,
}

impl SCSPErr {
    pub fn new(msg: &str) -> Self {
        SCSPErr {
            msg: String::from(msg),
        }
    }
}

impl From<ParseError> for SCSPErr {
    fn from(value: ParseError) -> Self {
        let msg = value.to_string().to_owned();
        SCSPErr { msg: msg }
    }
}

impl From<websocket::url::ParseError> for SCSPErr {
    fn from(value: websocket::url::ParseError) -> Self {
        let msg = value.to_string().to_owned();
        SCSPErr { msg: msg }
    }
}

impl From<WebSocketError> for SCSPErr {
    fn from(value: WebSocketError) -> Self {
        SCSPErr {
            msg: value.to_string(),
        }
    }
}
