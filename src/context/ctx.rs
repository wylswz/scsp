use std::sync::{atomic::AtomicBool, Arc, RwLock};

use crate::core::pubsub::{Bus, SimpleBus};

pub struct Context {
    pub bus: Arc<RwLock<dyn Bus>>,
    pub terminated: AtomicBool,
}

impl Context {
    pub fn init() -> Self {
        Context {
            bus: Arc::new(RwLock::new(SimpleBus::new())),
            terminated: AtomicBool::new(false),
        }
    }
}
