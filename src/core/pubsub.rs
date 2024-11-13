use std::sync::Arc;

use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct ChannelSummary {
    handlers: Vec<String>,
    channel: String,
}

#[allow(dead_code)]
pub struct MsgHandlerSummary {
    pub channels: Vec<ChannelSummary>,
}

pub trait Bus: Sync + Send {
    fn publish(&mut self, channel: String, msg: Vec<u8>);
    fn register_handler(&mut self, h: Arc<dyn MsgHandler>);
    fn list_handler(&self) -> MsgHandlerSummary;
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
        self.handlers.for_each_mut(channel, |h| {
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

    fn list_handler(&self) -> MsgHandlerSummary {
        let keys: Vec<String> = self.handlers.keys();
        let mut smry: Vec<ChannelSummary> = vec![];
        for k in keys {
            let mut subscriber_ids = vec![];
            self.handlers.for_each(k.clone(),  |v| {
                subscriber_ids.push(String::from(v.identity()));
            });
            smry.push(ChannelSummary{
                handlers: subscriber_ids,
                channel: k,
            });
        }
        MsgHandlerSummary{
            channels: smry
        }
    }
}

// tests

struct TestHandler {

}

impl MsgHandler for TestHandler {
    fn handle(&self, msg: Vec<u8>) -> Result<(), errs::SCSPErr> {
        todo!()
    }

    fn identity(&self) -> &str {
        return "1"
    }

    fn close(&self) {
        todo!()
    }

    fn is_closed(&self) -> bool {
        todo!()
    }
}


#[test]
fn test_register() {
    let mut bus = SimpleBus::new();
    bus.register_handler(Arc::new(TestHandler{}));

    let smry = bus.list_handler();
    assert_eq!(1, smry.channels.len());
    assert_eq!(1, smry.channels.get(0).unwrap().handlers.len())
    
}