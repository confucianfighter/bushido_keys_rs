// define event flags
use crate::conversion;
use crate::conversion::modifer_to_string_or_none;
use crate::hook_manager;
use crate::input_simulator;
use env_logger;
use lazy_static::lazy_static;
use log::warn;
use log::{debug, error, info};
use scopeguard::defer;
use scopeguard::guard;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyNameTextW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

pub const EVENT_FLAG_IS_POTENTIAL_MODE_ACTIVATION: u32 = 0x00000020;
pub const EVENT_FLAG_IS_KEY_MAPPING_EVENT: u32 = 0x00000040;
pub const EVENT_FLAG_IS_POTENTIAL_FIRST_MAPPING_KEY: u32 = 0x00000080;
pub const EVENT_FLAG_IS_SECOND_MAPPING_KEY: u32 = 0x00000100;
pub const EVENT_FLAG_IS_REPEAT_EVENT: u32 = 0x00000200;
pub const READY_TO_SEND_EVENT: u32 = 0x00000400;

lazy_static::lazy_static! {
    pub static ref EVENTS_BY_KEY_CODE: Mutex<HashMap<u32, Vec<Event>>> = Mutex::new(HashMap::new());
}

pub fn extract_kb_data(l_param: LPARAM) -> KBDLLHOOKSTRUCT {
    unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) }
}
pub fn get_char_from_vk_code(vk_code: u32) -> char {
    if let Some(key_name) = conversion::vk_to_string(vk_code) {
        key_name.chars().next().unwrap()
    } else {
        error!("Could not get key name for key code: {}", vk_code);
        '!'
    }
}
// Todo: press all keys, get value returned from each.
pub fn get_key_name_text(l_param: LPARAM) -> String {
    let mut buffer: [u16; 256] = [0; 256];
    unsafe { GetKeyNameTextW(l_param.0 as i32, &mut buffer) };
    String::from_utf16_lossy(&buffer[..buffer.iter().position(|&c| c == 0).unwrap()])
}
pub fn extract_vk_code_from_kb_data(kb_data: KBDLLHOOKSTRUCT) -> u32 {
    kb_data.vkCode
}
pub fn extract_modifiers_from_kb_data(kb_data: KBDLLHOOKSTRUCT) -> u32 {
    kb_data.flags.0
}
pub fn is_repeat(kb_data: KBDLLHOOKSTRUCT) -> bool {
    (kb_data.flags.0 & 0x40000000) != 0
}
pub fn forward_event(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}
#[derive(Clone)]
pub struct Event {
    my_flags: u32,
    timePressed: Instant,
    key_code_if_remap: Option<u32>,
    key_code: u32,
    modifiers: u32,
    l_param: LPARAM,
    w_param: WPARAM,
    n_code: i32,
}
impl Event {
    pub fn new(l_param: LPARAM, w_param: WPARAM, n_code: i32) -> Self {
        let message = w_param.0 as u32;
        let kb_data = extract_kb_data(l_param);
        let key_code = extract_vk_code_from_kb_data(kb_data);
        let modifiers = extract_modifiers_from_kb_data(kb_data);
        let mut my_flags = 0;
        let kb_data = extract_kb_data(l_param);
        Self {
            my_flags: my_flags,
            timePressed: Instant::now(),
            key_code_if_remap: None,
            key_code: key_code,
            modifiers: modifiers,
            l_param: l_param,
            w_param: w_param,
            n_code: n_code,
        }
    }
    pub fn get_key_code(&self) -> u32 {
        self.key_code
    }
    pub fn get_message(&self) -> u32 {
        self.w_param.0 as u32
    }
    pub fn is_key_down(&self) -> bool {
        self.get_message() == WM_KEYDOWN || self.get_message() == WM_SYSKEYDOWN
    }
    pub fn is_key_up(&self) -> bool {
        self.get_message() == WM_KEYUP || self.get_message() == WM_SYSKEYUP
    }
    pub fn is_potential_mode_activation(&self) -> bool {
        self.my_flags & EVENT_FLAG_IS_POTENTIAL_MODE_ACTIVATION != 0
    }
    pub fn is_key_mapping_event(&self) -> bool {
        self.my_flags & EVENT_FLAG_IS_KEY_MAPPING_EVENT != 0
    }
    pub fn is_potential_first_mapping_key(&self) -> bool {
        self.get_windows_flags() & EVENT_FLAG_IS_POTENTIAL_FIRST_MAPPING_KEY != 0
    }
    pub fn get_event_flags(&self) -> u32 {
        self.get_windows_flags()
    }
    pub fn is_ready_to_send(&self) -> bool {
        self.get_windows_flags() & READY_TO_SEND_EVENT != 0
    }
    pub fn get_windows_flags(&self) -> u32 {
        let kb_data = extract_kb_data(self.l_param);
        kb_data.flags.0
    }
    pub fn set_remap_key_code(&mut self, key_code: u32) {
        self.key_code_if_remap = Some(key_code);
    }
    pub fn get_remap_key_code(&self) -> Option<u32> {
        self.key_code_if_remap
    }
    pub fn get_modifiers(&self) -> u32 {
        self.modifiers
    }
    pub fn set_modifiers(&mut self, modifiers: u32) {
        self.modifiers = modifiers;
    }
    pub fn set_is_key_mapping_event(&mut self, is_key_mapping_event: bool) {
        self.my_flags = self.my_flags | EVENT_FLAG_IS_KEY_MAPPING_EVENT;
    }
    pub fn set_is_potential_mode_activation(&mut self, is_potential_mode_activation: bool) {
        self.my_flags = self.my_flags | EVENT_FLAG_IS_POTENTIAL_MODE_ACTIVATION;
    }
    pub fn set_is_potential_first_mapping_key(&mut self, is_potential_first_mapping_key: bool) {
        self.my_flags = self.my_flags | EVENT_FLAG_IS_POTENTIAL_FIRST_MAPPING_KEY;
    }
    pub fn is_potential_second_mapping_key(&self) -> bool {
        self.my_flags & EVENT_FLAG_IS_SECOND_MAPPING_KEY != 0
    }
    pub fn set_is_potential_second_mapping_key(&mut self, is_potential_second_mapping_key: bool) {
        self.my_flags = self.my_flags | EVENT_FLAG_IS_SECOND_MAPPING_KEY;
    }
    pub fn get_is_repeat(&self) -> bool {
        // see if event history contains this event by holding the lock guard
        let events_by_key_code_guard = EVENTS_BY_KEY_CODE.lock().unwrap();
        if let Some(history) = events_by_key_code_guard.get(&self.key_code) {
            //
            // get matching key code plus a match if it's key down or key up
            let matching_event = history.iter().find(|event| {
                event.key_code == self.key_code && event.is_key_down() == self.is_key_down()
            });
            if matching_event.is_some() {
                return true;
            }
        }
        false
    }
    pub fn set_is_ready_to_send(&mut self, is_ready_to_send: bool) {
        self.my_flags = self.my_flags | READY_TO_SEND_EVENT;
    }
    pub fn set_time_pressed(&mut self, time_pressed: Instant) {
        self.timePressed = time_pressed;
    }
    pub fn get_time_pressed(&self) -> Instant {
        self.timePressed
    }
    pub fn time_elapsed_since_key_down(&self) -> Duration {
        self.timePressed.elapsed()
    }
    pub fn get_from_event_history_or_self(&self) -> Event {
        info!("Matching against Event.key_code: {}", self.key_code);
        let events_by_key_code_guard = EVENTS_BY_KEY_CODE.lock().unwrap();
        if let Some(events) = events_by_key_code_guard.get(&self.key_code) {
            info!("History length: {}", events.len());
            // get matching key code plus a match if it's key down or key up
            let matching_event = events.iter().find(|event| {
                event.key_code == self.key_code && event.is_key_down() == self.is_key_down()
            });

            if matching_event.is_some() {
                return matching_event.unwrap().clone();
            } else {
                return self.clone();
            }
        } else {
            return self.clone();
        }
    }
    pub fn get_event_type(&self) -> EventType {
        if self.is_key_down() {
            EventType::KeyDown
        } else {
            EventType::KeyUp
        }
    }
    pub fn get_all_with_matching_key_code(&self) -> Vec<Event> {
        let events_by_key_code_guard = EVENTS_BY_KEY_CODE.lock().unwrap();

        events_by_key_code_guard
            .get(&self.key_code)
            .unwrap()
            .clone()
            .into_iter()
            .filter(|event| event.is_key_down() == self.is_key_down())
            .collect()
    }
}
// return a valid result type
pub fn subscribe_to_keyboard_events(
    handler: unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT,
) -> Result<HHOOK, String> {
    unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None)
            .expect("Failed to get module handle")
            .into();
        let hook_handle = SetWindowsHookExW(WH_KEYBOARD_LL, Some(handler), Some(h_instance), 0)
            .expect("Failed to install keyboard hook");
        if hook_handle.is_invalid() {
            eprintln!("Failed to install keyboard hook.");
            return Err("Failed to install keyboard hook.".to_string());
        }
        Ok(hook_handle)
    }
}
pub fn start_message_loop(hook_handle: HHOOK) {
    unsafe {
        let mut msg = MaybeUninit::<MSG>::uninit();
        while GetMessageW(msg.as_mut_ptr(), None, 0, 0).into() {
            TranslateMessage(msg.as_ptr());
            DispatchMessageW(msg.as_ptr());
        }
        stop_message_loop(hook_handle);
    }
}
pub fn stop_message_loop(hook_handle: HHOOK) {
    unsafe {
        UnhookWindowsHookEx(hook_handle);
    }
}
enum EventType {
    KeyDown,
    KeyUp,
}

// test mod
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeat() {
        env_logger::init();
        info!("Starting test");
        let hook = hook_manager::subscribe_to_keyboard_events(handle_keyboard_event).unwrap();
        hook_manager::start_message_loop(hook);
        // subscribe to keyboard events
    }
    extern "system" fn handle_keyboard_event(
        n_code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        let mut event = Event::new(l_param, w_param, n_code);
        debug!("Event recieved!");
        let mut event = event.get_from_event_history_or_self();
        if event.is_key_down() || event.is_key_up() {
            // defer statements are called in reverse order of declaration
            defer! {
                debug!("Adding event to history in scope guard drop in defer guard.");

                EVENTS_BY_KEY_CODE
                    .lock()
                    .unwrap()
                    .entry(event.get_key_code())
                    .or_insert(Vec::new())
                    .push(event.clone());
            };
            defer! {
                debug!("It let me call defer twice!");
            }
            if event.get_is_repeat() {
                info!("Repeat event");
                if event.is_key_down() {
                    debug!(
                        "Key has been held for {:?}",
                        event.time_elapsed_since_key_down()
                    );
                }
                return LRESULT(0);
            }
            match event.get_event_type() {
                EventType::KeyDown => {
                    let ninja_emoji = "\u{1F916}";

                    if let Some(key_name) = conversion::vk_to_string(event.get_key_code()) {
                        info!("{} key pressed", key_name);
                    } else {
                        let hex_key_code = format!("{:X}", event.get_key_code());
                        error!("Could not get key name for key code: {hex_key_code}");
                    }
                }
                EventType::KeyUp => {
                    info!(
                        "<<<{}>>> key released.",
                        get_char_from_vk_code(event.get_key_code()),
                    );
                    // clear event flags
                }
            }
        }
        hook_manager::forward_event(n_code, w_param, l_param)
    }
}
