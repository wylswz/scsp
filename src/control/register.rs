use std::sync::{Arc, Condvar};
use std::time::Duration;

use crate::context::ctx::Context;
use crate::core::data::MsgResponse;
use crate::core::pubsub::MsgHandler;
use crate::core::{self, errs};
use rocket::serde::json::Json;
use rocket::State;

use std::sync::{Mutex as Mux, RwLock};

struct PoolingHandler {
    channel: Box<str>,
    msg: Arc<(RwLock<Vec<u8>>, Condvar, Mux<bool>)>,
    client_id: Box<str>,
    closed: RwLock<bool>,
}

impl PoolingHandler {
    fn poll(&self) -> Option<Vec<u8>> {
            if self.closed.read().unwrap().clone() {
                return None;
            }
            let ready = self.msg.2.lock().unwrap();

            let _u = self
                .msg
                .1
                .wait(ready)
                .unwrap();
            
            let vec = self.msg.0.read().unwrap();
            let vec_content = vec.clone();
            return Some(vec_content);
    }

    fn new(client_id: Box<str>, channel: Box<str>) -> Self {
        Self {
            client_id: client_id.clone(),
            channel: channel.clone(),
            msg: Arc::new((RwLock::new(vec![]), Condvar::new(), Mux::new(false))),
            closed: RwLock::new(false),
        }
    }
}

impl MsgHandler for PoolingHandler {
    fn handle(&self, msg: Vec<u8>) -> Result<(), errs::SCSPErr> {
        let _ = self.msg.0.write().map(|mut v| {
            v.clear();
            for b in msg {
                v.push(b);
            }
        });
        let mut ready = self.msg.2.lock().unwrap();
        *ready = true;
        info!("notifying watchers");
        self.msg.1.notify_all();
        Ok(())
    }

    fn identity(&self) -> &str {
        &self.client_id
    }

    fn channel(&self) -> &str {
        &self.channel
    }

    fn close(&self) {
        let mut closed_ref = self.closed.write().unwrap();
        *closed_ref = true;
    }

    fn is_closed(&self) -> bool {
        *self.closed.read().unwrap()
    }
}




/// register a websocket listener on message bus
/// block until
/// - application is shut down
/// - handler is removed from the bus
/// - message channel is deleted
#[get("/register?<client_id>&<channel>")]
#[allow(unused_variables)]
pub fn register<'r>(ctx: &State<Context>, client_id: &str, channel: &str) -> Json<MsgResponse> {
    let h: PoolingHandler = PoolingHandler::new(Box::from(client_id), Box::from(channel));
    let arc_h: Arc<PoolingHandler> = Arc::new(h);
    _ = ctx.bus.write().map(|mut b| {
        b.register_handler(arc_h.clone());
    });

    let res = arc_h.clone().poll();
    arc_h.clone().close();
    match res {
        None => Json::from(MsgResponse{has_msg: false, msg: vec![]}),
        Some(msg) => {
            // let _ = arc_h.clone().clone();
            Json::from(MsgResponse{has_msg: true, msg: msg})
        }
    }
}
