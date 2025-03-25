use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct KeyState {
    pub vk_code: i32,
    pub name: String,
    pub time_pressed: Instant,
    pub timeout: u32,
    pub held: bool,
    pub prev_held: bool,
    pub time_released: Instant,
    pub was_double_tap: bool,
    pub was_shift_held_on_key_down: bool,
}

impl Default for KeyState {
    fn default() -> Self {
        Self {
            vk_code: 0,
            name: "no name".to_string(),
            time_pressed: Instant::now(),
            timeout: 200,
            held: false,
            prev_held: false,
            time_released: Instant::now(),
            was_double_tap: false,
            was_shift_held_on_key_down: false,
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

// Virtual key codes for modifier keys
pub const VK_SHIFT: u32 = 0x10;
pub const VK_CONTROL: u32 = 0x11;
pub const VK_ALT: u32 = 0x12;
pub const VK_LSHIFT: u32 = 0xA0;
pub const VK_RSHIFT: u32 = 0xA1;
pub const VK_LCONTROL: u32 = 0xA2;
pub const VK_RCONTROL: u32 = 0xA3;
pub const VK_LALT: u32 = 0xA4;
pub const VK_RALT: u32 = 0xA5;

impl KeyState {
    pub fn new(vk_code: i32) -> Self {
        Self {
            vk_code,
            time_pressed: Instant::now(),
            time_released: Instant::now(),
            held: false,
            name: "no name".to_string(),
            timeout: 200,
            prev_held: false,
            was_double_tap: false,
            was_shift_held_on_key_down: false,
        }
    }

    /// Check if this key is a modifier key
    pub fn is_modifier(&self) -> bool {
        let code = self.vk_code as u32;
        matches!(
            code,
            VK_SHIFT
                | VK_CONTROL
                | VK_ALT
                | VK_LSHIFT
                | VK_RSHIFT
                | VK_LCONTROL
                | VK_RCONTROL
                | VK_LALT
                | VK_RALT
        )
    }
}

/// Get the current state of all modifier keys
pub fn get_active_modifiers() -> Vec<u32> {
    let mut modifiers = Vec::new();
    let states = KEY_STATES.lock().unwrap();

    // Check each modifier key
    for &vk_code in &[
        VK_SHIFT as i32,
        VK_CONTROL as i32,
        VK_ALT as i32,
        VK_LSHIFT as i32,
        VK_RSHIFT as i32,
        VK_LCONTROL as i32,
        VK_RCONTROL as i32,
        VK_LALT as i32,
        VK_RALT as i32,
    ] {
        if let Some(state) = states.get(&vk_code) {
            if state.lock().unwrap().held {
                modifiers.push(vk_code as u32);
            }
        }
    }

    modifiers
}

/// Convert left/right specific modifiers to their general form
pub fn normalize_modifier(vk_code: u32) -> u32 {
    match vk_code {
        VK_LSHIFT | VK_RSHIFT => VK_SHIFT,
        VK_LCONTROL | VK_RCONTROL => VK_CONTROL,
        VK_LALT | VK_RALT => VK_ALT,
        _ => vk_code,
    }
}
