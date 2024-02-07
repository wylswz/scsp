use std::sync::{Arc, Mutex};

use crate::core::pubsub::{Bus, SimpleBus};

pub struct Context {
    pub bus: Arc<Mutex<dyn Bus>>
}

impl Context {
    pub fn init() -> Self{
        Context {
            bus: Arc::new(Mutex::new(SimpleBus::new())),
        }
    }
}