use std::borrow::BorrowMut;
use std::sync::{Arc, Condvar};
use std::time::Duration;

use crate::context::ctx::Context;
use crate::core::errs;
use crate::core::pubsub::MsgHandler;
use rocket::time::Instant;
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
}

impl WebsocketHandler {
    fn poll(&self) -> Option<Vec<u8>> {
        let res = self
            .msg
            .2
            .lock()
            .map(|ready: std::sync::MutexGuard<'_, bool>| {
                let mut wait_res = self
                    .msg
                    .1
                    .wait_timeout(ready, Duration::from_secs(1))
                    .unwrap();

                if wait_res.1.timed_out() {
                    return None;
                }
                let vec = self.msg.0.lock().unwrap();

                let vec_content = vec.clone();
                *wait_res.0 = false;
                Some(vec_content)
            });
        res.unwrap()
    }

    fn new(client_id: Box<str>, channel: Box<str>) -> Self {
        Self {
            client_id: client_id.clone(),
            channel: channel.clone(),
            msg: Arc::new((Mux::new(vec![]), Condvar::new(), Mux::new(false))),
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
}

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
    _ = ctx.bus.lock().and_then(|mut b| {
        b.register_handler(arc_h.clone());
        Ok(())
    });
    let mut interval = time::interval(Duration::from_millis(20));
    Stream! { ws =>
        loop {
            select! {
                _ = interval.tick()  => {
                    let nxt_msg_res = arc_h.clone().borrow_mut().poll();
                    // pull message from the handler
                    match nxt_msg_res {
                        Some(nxt_msg) => {
                            yield Message::text(String::from_utf8(nxt_msg).unwrap());
                        },
                        _ => {}
                    }
                },
                _ = &mut shutdown => {
                    break;
                }
            }
        }
    }
}
