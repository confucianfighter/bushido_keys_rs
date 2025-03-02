// src/key_state.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

#[derive(Debug, Clone)]
pub struct KeyState {
    pub vk_code: i32,
    pub name: String,
    pub time_pressed: u128,
    pub timeout: u32,
    pub held: bool,
    pub prev_held: bool,
    pub time_released: u128,
}

impl Default for KeyState {
    fn default() -> Self {
        Self {
            vk_code: 0,
            name: "no name".to_string(),
            time_pressed: 0,
            timeout: 200,
            held: false,
            prev_held: false,
            time_released: 0,
        }
    }
}

pub type SafeKeyState = Arc<Mutex<KeyState>>;

pub static KEY_STATES: Lazy<Mutex<HashMap<i32, SafeKeyState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_debug(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::SeqCst);
}

pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if $crate::key_state::is_debug_enabled() {
            println!($($arg)*);
        }
    }
}