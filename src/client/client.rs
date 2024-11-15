use std::{borrow::BorrowMut, cell::{Cell, RefCell}, fmt::Debug, future::IntoFuture, io::ErrorKind, sync::RwLock, thread, time::Duration};

use log::info;
use rocket::http;
use url::Url;

use crate::core::{data::MsgResponse, errs::SCSPErr};

pub trait Client {
    /// register a handler on a channel, keep reading from websocket, until
    /// connection is closed (by another side)
    /// cancelled by invoker (by setting sig to a non-zero value)
    async fn register(
        &self,
        client_id: &str,
        channel: &str,
        handler: impl Fn(Vec<u8>) -> Result<(), SCSPErr>,
        sig: RwLock<i32>,
    ) -> Result<(), SCSPErr>;
    fn write(&self);
}

pub struct DefaultClient {
    host: Url,
}

impl DefaultClient {
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
            host: host_url,
        })
    }
}

impl Client for DefaultClient {
    async fn register(
        &self,
        client_id: &str,
        channel: &str,
        handler: impl Fn(Vec<u8>) -> Result<(), SCSPErr>,
        sig: RwLock<i32>
    ) -> Result<(), SCSPErr> {
        let mut ep = self.host.to_string();

        ep.push_str("/register");
        let mut url = Url::parse(ep.as_str()).unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", client_id)
            .append_pair("channel", channel);
        loop {
            info!("registering watch");
            let resp = reqwest::get(url.clone()).await.unwrap().json::<MsgResponse>().await.unwrap();
            info!("resp {}", resp.has_msg);
            if resp.has_msg {
                let res = handler(resp.msg);
                if res.is_err() {
                    return res
                }
            }
        }
        Ok(())

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
