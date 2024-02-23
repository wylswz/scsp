use rocket::http;
use url::Url;
use websocket::{ClientBuilder, OwnedMessage};

use crate::core::errs::SCSPErr;

pub trait Client {
    fn register(
        &self,
        client_id: &str,
        channel: &str,
        handler: impl Fn(Vec<u8>) -> Result<(), SCSPErr>,
    ) -> Result<(), SCSPErr>;
    fn write(&self);
}

pub struct DefaultClient {
    host: Url,
    ws_host: Url,
}

impl DefaultClient {
    fn infer_ws_host(host: &Url) -> Url {
        let mut res = host.clone();
        let _ = res.set_scheme(match host.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => panic!("should't happen here"),
        });
        res
    }

    pub fn new(host: &str) -> Result<DefaultClient, SCSPErr> {
        let res = Url::parse(host);
        if res.is_err() {
            return Err(res.err().unwrap().into());
        }
        let host_url = res.unwrap();
        if host_url.scheme() != "http" && host_url.scheme() != "https" {
            return Err(SCSPErr::new("unsupported scheme"));
        }
        Ok(DefaultClient {
            ws_host: Self::infer_ws_host(&host_url),
            host: host_url,
        })
    }
}

impl Client for DefaultClient {
    fn register(
        &self,
        client_id: &str,
        channel: &str,
        handler: impl Fn(Vec<u8>) -> Result<(), SCSPErr>,
    ) -> Result<(), SCSPErr> {
        let mut ep = self.ws_host.to_string();

        ep.push_str("/register");
        let mut url = Url::parse(ep.as_str()).unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", client_id)
            .append_pair("channel", channel);
        let builder_res = ClientBuilder::new(url.as_str());
        if builder_res.is_err() {
            return Err(builder_res.err().unwrap().into());
        }

        let conn_res = builder_res.unwrap().connect_insecure();
        if conn_res.is_err() {
            return Err(SCSPErr::from(conn_res.err().unwrap()));
        }
        let mut conn = conn_res.unwrap();
        loop {
            let res = match conn.recv_message() {
                Ok(OwnedMessage::Binary(b)) => handler(b),
                _ => Err(SCSPErr::new("expected binary message")),
            };
            if res.is_err() {
                return res;
            }
        }
    }

    fn write(&self) {
        todo!()
    }
}

#[test]
fn test_client_url() {
    let res = DefaultClient::new("http://127.0.0.1:6872");
    assert!(res.is_ok());
    assert!(DefaultClient::new("ftp://127.0.0.1:6872").is_err());
    assert!(DefaultClient::new("ftp://127.0.0.1:asd:asd:6872").is_err());
}
