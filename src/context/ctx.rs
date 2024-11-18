use std::sync::{atomic::AtomicBool, Arc, RwLock};


pub struct Context {
    pub terminated: AtomicBool,
}

impl Context {
    pub fn init() -> Self {
        Context {
            terminated: AtomicBool::new(false),
        }
    }
}
