use std::sync::Arc;

use super::cmm::ConcurrentMultiMap;
use super::errs;
use std::marker::Send;

pub trait MsgHandler: Sync + Send {
    fn handle(&self, msg: Vec<u8>) -> Result<(), errs::SCSPErr>;
    fn identity(&self) -> &str;
    fn channel(&self) -> &str {
        "default"
    }
    fn close(&self);
    fn is_closed(&self) -> bool;
}

#[allow(dead_code)]
pub struct ChannelSummary<'r> {
    handlers: Vec<&'r str>,
    channel: &'r str,
}

#[allow(dead_code)]
pub struct MsgHandlerSummary<'r> {
    channels: Vec<ChannelSummary<'r>>,
}

pub trait Bus: Sync + Send {
    fn publish(&mut self, channel: String, msg: Vec<u8>);
    fn register_handler(&mut self, h: Arc<dyn MsgHandler>);
    fn list_handler(&mut self) -> MsgHandlerSummary;
}

pub struct SimpleBus {
    handlers: ConcurrentMultiMap<String, Arc<dyn MsgHandler>>,
}

impl SimpleBus {
    pub fn new() -> Self {
        SimpleBus {
            handlers: ConcurrentMultiMap::new(),
        }
    }
}

impl Bus for SimpleBus {
    fn publish(&mut self, channel: String, msg: Vec<u8>) {
        // lazily remove closed handlers
        self.handlers
            .remove_if(channel.to_owned(), |v| v.is_closed());
        self.handlers.for_each(channel, |h| {
            // TODO: log
            let _ = h.handle(msg.clone());
        })
    }

    fn register_handler(&mut self, h: Arc<dyn MsgHandler>) {
        self.handlers
            .append_if_absent(h.channel().to_string(), h, |v1, v2| {
                v1.identity() == v2.identity()
            });
    }

    fn list_handler(&mut self) -> MsgHandlerSummary {
        todo!()
    }
}
