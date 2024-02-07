use std::borrow::BorrowMut;
use std::sync::{Arc, Condvar};
use std::time::Duration;

use crate::context::ctx::Context;
use crate::core::errs;
use crate::core::pubsub::MsgHandler;
use rocket::tokio::select;
use rocket::tokio::time::{self};
use rocket::Shutdown;
use rocket::State;
use rocket_ws::WebSocket;
use rocket_ws::{Message, Stream};

use std::sync::Mutex as Mux;

struct WebsocketHandler {
    channel: Box<str>,
    msg: Arc<(Mux<Vec<u8>>, Condvar, Mux<bool>)>,
    client_id: Box<str>,
    closed: bool,
}

impl WebsocketHandler {
    fn poll(&self) -> Option<Vec<u8>> {
        let ready = self.msg.2.lock().unwrap();

        let mut wait_res = self
            .msg
            .1
            .wait_timeout(ready, Duration::from_secs(2))
            .unwrap();

        if wait_res.1.timed_out() {
            return None;
        }

        *wait_res.0 = false;
        let vec = self.msg.0.lock().unwrap();

        let vec_content = vec.clone();
        Some(vec_content)
    }

    fn new(client_id: Box<str>, channel: Box<str>) -> Self {
        Self {
            client_id: client_id.clone(),
            channel: channel.clone(),
            msg: Arc::new((Mux::new(vec![]), Condvar::new(), Mux::new(false))),
            closed: false,
        }
    }
}

impl MsgHandler for WebsocketHandler {
    fn handle(&self, msg: Vec<u8>) -> Result<(), errs::SCSPErr> {
        let _ = self.msg.0.lock().map(|mut v| {
            v.clear();
            for b in msg {
                v.push(b);
            }
        });
        let mut ready = self.msg.2.lock().unwrap();
        *ready = true;
        self.msg.1.notify_all();
        Ok(())
    }

    fn identity(&self) -> &str {
        &self.client_id
    }

    fn channel(&self) -> &str {
        &self.channel
    }

    fn close(&mut self) {
        self.closed = true;
    }
}

/// register a websocket listener on message bus
/// block until
/// - application is shut down
/// - handler is removed from the bus
/// - message channel is deleted
#[get("/register?<client_id>&<channel>")]
#[allow(unused_variables)]
pub fn register<'r>(
    ctx: &State<Context>,
    client_id: &str,
    channel: &str,
    ws: WebSocket,
    mut shutdown: Shutdown,
) -> Stream!['static] {
    let h = WebsocketHandler::new(Box::from(client_id), Box::from(channel));
    let arc_h = Arc::new(h);
    _ = ctx.bus.write().map(|mut b| {
        b.register_handler(arc_h.clone());
        drop(b);
    });
    let mut interval = time::interval(Duration::from_millis(200));
    Stream! { ws =>
        loop {
            if arc_h.clone().borrow_mut().closed {
                break;
            }
            // pull message from the handler
            match arc_h.clone().borrow_mut().poll() {
                Some(nxt_msg) => {
                    yield Message::binary(nxt_msg);
                },
                _ => { /* timeout */ }
            }
        }
    }
}
