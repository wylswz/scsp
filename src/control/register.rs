use std::borrow::BorrowMut;
use std::future::IntoFuture;
use std::sync::{Arc, Condvar};
use std::time::Duration;

use crate::context::ctx::Context;
use crate::core::consts::MSG_PING;
use crate::core::pubsub::MsgHandler;
use crate::core::{self, errs};
use log::info;
use rocket::futures::future::BoxFuture;
use rocket::futures::stream::FusedStream;
use rocket::futures::{SinkExt, TryStreamExt};
use rocket::tokio::select;
use rocket::tokio::time::{self};
use rocket::Shutdown;
use rocket::State;
use rocket_ws::{Channel, WebSocket};
use rocket_ws::{Message, Stream};

use std::sync::{Mutex as Mux, RwLock};

struct WebsocketHandler {
    channel: Box<str>,
    msg: Arc<(Mux<Vec<u8>>, Condvar, Mux<bool>)>,
    client_id: Box<str>,
    closed: RwLock<bool>,
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
            closed: RwLock::new(false),
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
pub fn register<'r>(
    ctx: &State<Context>,
    client_id: &str,
    channel: &str,
    ws: WebSocket,
    mut shutdown: Shutdown,
) -> Channel<'static> {
    let h = WebsocketHandler::new(Box::from(client_id), Box::from(channel));
    let arc_h = Arc::new(h);
    _ = ctx.bus.write().map(|mut b| {
        b.register_handler(arc_h.clone());
        drop(b);
    });

    ws.channel(|mut c| {
        Box::pin(async move {
            loop {
                if c.is_terminated() {
                    arc_h.clone().close();
                }

                if arc_h.clone().is_closed() {
                    break;
                }

                // pull message from the handler
                match arc_h.clone().borrow_mut().poll() {
                    Some(nxt_msg) => {
                        let send_res = c.send(Message::binary(nxt_msg)).await;
                        match send_res {
                            Err(_) => {
                                arc_h.clone().close();
                            }
                            _ => {}
                        };
                    }
                    _ => { /* timeout */ }
                }
            }
            Ok(())
        })
    })
}
