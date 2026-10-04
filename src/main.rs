use std::time::{Duration, Instant};

extern "C" {
    fn initialize_vectorium();
    fn tick_vectorium();
}

fn main() {
    unsafe {
        initialize_vectorium();
    }

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        if last_tick.elapsed() >= tick_rate {
            unsafe {
                tick_vectorium();
            }
            last_tick = Instant::now();
        }
    }
}
