use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct KeyState {
    pub vk_code: i32,
    pub name: String,
    pub time_pressed: u128,
    pub timeout: u32,
    pub held: bool,
    pub prev_held: bool,
    pub time_released: u128,
}

// Define type alias so that each key state is individually locked.
pub type SafeKeyState = Arc<Mutex<KeyState>>;

pub static KEY_STATES: Lazy<Mutex<HashMap<i32, SafeKeyState>>> = Lazy::new(|| Mutex::new(HashMap::new())); 