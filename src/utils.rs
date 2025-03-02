use std::sync::Once;
use std::time::Instant;

// Added a static initializer for the base time
static INIT: Once = Once::new();
static mut BASE_TIME: Option<Instant> = None;

/// Returns the current time in milliseconds relative to when the program started.
pub fn current_time_ms() -> u128 {
    INIT.call_once(|| unsafe {
        BASE_TIME = Some(Instant::now());
    });
    let base = unsafe { BASE_TIME.unwrap() };
    Instant::now().duration_since(base).as_millis()
}
