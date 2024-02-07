use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::core::pubsub::{Bus, SimpleBus};

pub struct Context {
    pub bus: Arc<Mutex<dyn Bus>>,
    pub terminated: AtomicBool
}

impl Context {
    pub fn init() -> Self{
        Context {
            bus: Arc::new(Mutex::new(SimpleBus::new())),
            terminated: AtomicBool::new(false)
        }
    }
}