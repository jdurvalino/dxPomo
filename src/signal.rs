use std::sync::{
    atomic::{AtomicBool, Ordering},
};

static CANCELLED: AtomicBool = AtomicBool::new(false);

pub fn init() {
    ctrlc::set_handler(|| {
        CANCELLED.store(true, Ordering::SeqCst);
    })
    .expect("Failed to set Ctrl+C handler");
}

pub fn is_cancelled() -> bool {
    CANCELLED.load(Ordering::SeqCst)
}
