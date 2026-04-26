// direitos autorais (Rust) DCrust 16/04/2026
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let buffer_overflow: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let memory_leaks: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    loop {
        let bof = buffer_overflow.load(Ordering::SeqCst);
        let ml = memory_leaks.load(Ordering::SeqCst);

        if bof { break; }
        if ml  { break; }

        break;
    }
}
