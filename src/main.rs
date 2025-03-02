use std::collections::HashMap;
use std::fs;
use std::mem::MaybeUninit;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use serde_json;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use env_logger;
use log::{debug, error, info, warn};
use tracelogging::Provider;

mod basic_mode;
mod conversion;
mod input_simulator;
mod key_and_modifiers;
mod key_state;
mod mode;
mod mode_config;
mod mouse_mode;
mod utils;
use basic_mode::BasicMode;
use key_state::{KeyState, KEY_STATES};
use mode::Mode;
use mode_config::{ModeConfig, ModesConfig};
use mouse_mode::MouseMode;
use utils::*;

static mut HOOK_HANDLE: HHOOK = HHOOK(std::ptr::null_mut());

/// Global active mode is optional.
static CURRENT_MODE: Lazy<Mutex<Option<Box<dyn Mode + Send>>>> = Lazy::new(|| Mutex::new(None));
/// Available modes loaded from configuration.
static AVAILABLE_MODES: Lazy<Mutex<Vec<Box<dyn Mode + Send>>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

// Global counters
static KEY_DOWN_EVENTS: AtomicUsize = AtomicUsize::new(0);
static MODE_ACTIVATIONS: AtomicUsize = AtomicUsize::new(0);
static BLOCKED_KEYS: AtomicUsize = AtomicUsize::new(0);

/// The low-level keyboard hook procedure.
/// - It tracks key states in the KEY_STATES global.
/// - It passes a KeyState instance to mode event handlers.
/// create a function that takes care of lose ends before returning a result
fn handle_lose_ends(
    current_mode: Option<Box<dyn Mode + Send>>,
    state: &mut KeyState,
    propogate: bool,
) -> LRESULT {
    *CURRENT_MODE.lock().unwrap() = current_mode;
    // get the kv_code
    let kv_code = state.vk_code;
    // lock states and update or insert the state for this vk_code by cloning and wrapping in Arc<Mutex<_>>
    let mut states = KEY_STATES.lock().unwrap();
    states
        .entry(kv_code)
        .and_modify(|e: &mut std::sync::Arc<Mutex<KeyState>>| {
            *e = std::sync::Arc::new(std::sync::Mutex::new(state.clone()))
        })
        .or_insert(std::sync::Arc::new(std::sync::Mutex::new(state.clone())));
    if propogate {
        return LRESULT(0);
    }
    return LRESULT(1);
}

extern "system" fn keyboard_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let message = w_param.0 as u32;
    let is_key_down = message == WM_KEYDOWN || message == WM_SYSKEYDOWN;
    let is_key_up = message == WM_KEYUP || message == WM_SYSKEYUP;
    // if it's a key up, create a distinguished message with an emoticon that this is a key up event, perhaps an up arrow
    // if it's a key down, create a distinguished message with an emoticon that this is a key down event, perhaps a down arrow
    if is_key_up {
        // add emojis to the message
        info!("👆 Key up event detected");
    } else if is_key_down {
        info!("👇 Key down event detected");
    }
    if message != WM_KEYDOWN
        && message != WM_KEYUP
        && message != WM_SYSKEYDOWN
        && message != WM_SYSKEYUP
    {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
    }

    // Extract kb_data once safely
    let kb_data = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let vk_code = kb_data.vkCode;
    // get char from vk_code
    let char = input_simulator::get_char_from_vk_code(vk_code);
    info!(
        "It was the {} key that was either pressed or released.",
        char
    );
    // We know to log the key down event if it activates a mode, or if there is an active mode, and it's in the keymap.

    // Here are the possible outcomes:
    // 1. No mode is active, and the key is not in the keymap of any mode. The key is forwarded.
    // 2. There is an active mode, the key is in the keymap. The event is not propogated, and instead a key up or key down is simulated.
    // 3. There is an active mode, its a key up event, the key matches the activation key. The mode is deactivated.
    // 4. There is an active mode, its a key up event, the key does not match the activation key. The event is forwarded.
    // 5. There is an active mode, the key is in the keymap of a mode. The mode is activated, and the key is not propagated.
    //
    // Perhaps it would be more productive to think in terms of the set of individual actions, and then contraints
    // 1. Log the keystate
    // 2. Simulate a key down or key up event
    // 3. Simulate a mouse click

    // necessary data:
    // 1. Current key state.
    // 2. hashmap of key states.
    // 2. Optional current mode.
    // 3. Activation key of current mode
    // 4. Time elapsed since key down of activation key
    // 5. List of modes.

    // Conditions:
    // Forward immediately if it's not a key up or key down event.
    // In all cases of a key up or key down event, update the key state.
    // Key Up:
    //      Update the key state with prev state and current state.
    //      If the key matches the activation key, and the mode is active,
    //         Deactivate the mode.
    //         Do not forward the event.
    //         Check the timeout. If the key was held for less than timeout, simulate key press of activation key.
    //      If there is an active mode, run the event through the key up event handler of the mode.
    //      If there is no active mode, forward the event.
    // Key Down:
    //      If prev state is held
    //         If there is an active mode, do not process the event.
    //         We know it doesn't activate a mode, becauwse it's already held.
    //      If prev state is not held,
    //         update the key state with prev state and current state. mark the time pressed.
    //         If there is an active mode, run the event through the key down event handler of the mode.
    //         simplify things by not forwarding the event if there is an active mode, or if it activates or deactivates a mode.
    //
    //
    let mut state = {
        let mut key_states = KEY_STATES.lock().unwrap();
        key_states
            .entry(vk_code as i32)
            .or_insert_with(|| {
                std::sync::Arc::new(Mutex::new(KeyState {
                    vk_code: vk_code as i32,
                    name: format!("VK_{:X}", vk_code),
                    time_pressed: current_time_ms() as u128,
                    timeout: 200,
                    held: is_key_down,
                    prev_held: false,
                    time_released: 0,
                }))
            })
            .clone()
    };
    let mut state = state.lock().unwrap().clone();
    // about to lock and clone the state arc.
    state.held = is_key_down;
    let is_repeat = state.held && state.prev_held;
    info!("prev_held: {}", state.prev_held);
    state.prev_held = state.held;
    if !is_repeat && is_key_down {
        state.time_pressed = current_time_ms() as u128;
    }
    // make a copy of the current mode
    // add lock emoji to the message
    info!("🔒 About to clone the current mode, will there be a deadlock?");
    let mut current_mode = CURRENT_MODE.lock().unwrap().clone();
    info!("🔓 Just cloned the current mode");
    // here we go. Master branch, is there or is there not a mode active
    if let Some(mode) = current_mode.as_mut() {
        info!("We are at the top of the block where there is a mode active");
        if is_repeat {
            info!("It' repeat therefore next line of code returns LRESULT(1) because we don't wish to propogate the repeat event");
            return handle_lose_ends(Some(mode.clone()), &mut state, false);
        }
        if is_key_down && !is_repeat {
            state.time_pressed = current_time_ms() as u128;
            info!("There's a mode active and a key is down and it's not a repeat");
            if mode.handle_key_down_event(&mut state) {
                info!("mode handled key down event and returned true to main.");
                // do not forward the event
                return handle_lose_ends(Some(mode.clone()), &mut state, false);
            } else {
                info!("mode did not handle key down event and returned false to main.");
                return handle_lose_ends(Some(mode.clone()), &mut state, true);
            }
        } else if is_key_up {
            let char = input_simulator::get_char_from_vk_code(vk_code);
            // add skull emoji to the message
            info!(
                "💀 💀 💀  There's a mode active and a key is up, it was the {} key",
                char
            );
            // check if the key matches the activation key
            if mode.check_if_deactivates(&mut state) {
                info!("The key matches the activation key and the mode will be deactivated");
                // set current mode to none
                // deactivate the mode
                // set current mode to none
                // drop current mode to unlock it
                // check time_pressed against now
                // subtract with overflow
                let elapsed_millis = current_time_ms().abs_diff(state.time_pressed);
                info!("Elapsed time since key down: {}ms", elapsed_millis);
                if elapsed_millis < 200 {
                    // simulate key press of activation key
                    info!("Simulating key tap of activation key");
                    input_simulator::simulate_key_tap(vk_code, &[]);
                    return handle_lose_ends(None, &mut state, false);
                } else {
                    info!("Key was held for more than 200ms, so not simulating key tap");
                    return handle_lose_ends(None, &mut state, true);
                }
            } else if mode.handle_key_up_event(&mut state) {
                return handle_lose_ends(Some(mode.clone()), &mut state, false);
            } else {
                info!("forwarding the event because handle_key_up_event returned false");
                return handle_lose_ends(Some(mode.clone()), &mut state, true);
            }
        } else {
            info!("didn't handle the key down event, forwarding the event, this logic is questionable");
            // forward the event
            return handle_lose_ends(Some(mode.clone()), &mut state, true);
        }
    } else {
        // no current mode
        if is_key_down && !is_repeat {
            //state_arc.lock().unwrap().time_pressed = current_time_ms() as u128;
            // check if any mode is activated by this key
            for mode in AVAILABLE_MODES.lock().unwrap().iter_mut() {
                if mode.get_activation_keys().contains(&vk_code) {
                    mode.set_activated_by(vk_code);
                    state.time_pressed = current_time_ms();
                    info!("Detected a key down, it matches an activation key. Setting current mode to {}", mode.get_name());

                    // set the activated_by variable
                    info!("Current mode set to {}", mode.get_name());
                    return handle_lose_ends(Some(mode.clone()), &mut state, false);
                }
            }
        }
    }
    return handle_lose_ends(current_mode.clone(), &mut state, true);
}

fn main() {
    // Initialize logger with environment variables (RUST_LOG=debug, info, warn, error)
    env_logger::init();

    // Load available modes from configuration.
    let config_str = fs::read_to_string("modes.json")
        .expect("Failed to read modes.json in the working directory.");
    let modes_config: ModesConfig =
        serde_json::from_str(&config_str).expect("Failed to parse modes.json");

    let mut available_modes: Vec<Box<dyn Mode + Send>> = Vec::new();
    for mode_cfg in modes_config.modes {
        // For demonstration, create a BasicMode from each configuration.
        let mode_instance = BasicMode::new(mode_cfg);
        info!("Loaded mode: {}", mode_instance.config.get_name());
        info!("Activation keys: {:?}", mode_instance.get_activation_keys());
        available_modes.push(Box::new(mode_instance));
    }
    info!("Loaded modes config about to add mouse mode");
    available_modes.push(Box::new(MouseMode::new()));
    info!("Added mouse mode");
    // Store the available modes globally.
    *AVAILABLE_MODES.lock().unwrap() = available_modes;

    // Example: you might want to activate a mode based on a configuration or on-demand.
    // *current_mode = Some(AVAILABLE_MODES.lock().unwrap()[0].clone());

    // Update active mode thread
    thread::spawn(|| loop {
        if let Some(ref mut mode) = *CURRENT_MODE.lock().unwrap() {
            mode.update();
        }
        // Periodically report stats using ETW
        // info!(
        //     "Stats: {} key events, {} mode activations, {} keys blocked",
        //     KEY_DOWN_EVENTS.load(Ordering::Relaxed),
        //     MODE_ACTIVATIONS.load(Ordering::Relaxed),
        //     BLOCKED_KEYS.load(Ordering::Relaxed)
        // );
        thread::sleep(Duration::from_millis(1));
    });

    unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None)
            .expect("Failed to get module handle")
            .into();
        HOOK_HANDLE = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), Some(h_instance), 0)
            .expect("Failed to install keyboard hook");
        if HOOK_HANDLE.is_invalid() {
            eprintln!("Failed to install keyboard hook.");
            return;
        }
    }

    info!("Rust version running... Press ESC to exit.");

    // Windows message loop.
    unsafe {
        let mut msg = MaybeUninit::<MSG>::uninit();
        while GetMessageW(msg.as_mut_ptr(), None, 0, 0).into() {
            TranslateMessage(msg.as_ptr());
            DispatchMessageW(msg.as_ptr());
        }
        UnhookWindowsHookEx(HOOK_HANDLE);
    }
}

fn debug_out(msg: &str) {
    debug!("{}", msg);
}
