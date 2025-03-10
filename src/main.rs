use std::fs;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use once_cell::sync::Lazy;
use serde_json;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use env_logger;
use log::{debug, info};
use quote::quote;
mod basic_mode;
mod conversion;
mod event;
mod hook_manager;
mod input_simulator;
mod key_and_modifiers;
mod key_state;
mod mode;
mod mode_config;
mod mode_json;
mod mouse_config_json;
mod mouse_mode;
mod utils;
use basic_mode::BasicMode;
use key_state::{KeyState, KEY_STATES};
use mode::Mode;
use mode_config::ModesConfig;
use mouse_mode::MouseMode;
use std::env;
use std::path::Path;
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

    if is_key_up {
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
    let is_repeat = (kb_data.flags.0 & 0x40000000) != 0;

    // Create or update key state
    let mut state = KeyState::new(vk_code as i32);
    state.held = is_key_down;

    if is_key_down {
        state.time_pressed = current_time_ms() as u128;
    } else {
        state.time_released = current_time_ms() as u128;
    }

    // Get char from vk_code for logging
    let char = input_simulator::get_char_from_vk_code(vk_code);
    let active_modifiers = key_state::get_active_modifiers();

    // Log the event with modifier information
    if !active_modifiers.is_empty() {
        let modifier_names: Vec<String> = active_modifiers
            .iter()
            .map(|&m| match key_state::normalize_modifier(m) {
                key_state::VK_SHIFT => "SHIFT".to_string(),
                key_state::VK_CONTROL => "CTRL".to_string(),
                key_state::VK_ALT => "ALT".to_string(),
                _ => format!("MOD_{:X}", m),
            })
            .collect();

        info!(
            "{} key {} with modifiers: [{}]",
            char,
            if is_key_down { "pressed" } else { "released" },
            modifier_names.join("+")
        );
    } else {
        info!(
            "{} key {}",
            char,
            if is_key_down { "pressed" } else { "released" }
        );
    }

    // Get current mode (if any)
    let mut current_mode = CURRENT_MODE.lock().unwrap().take();

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
    if let Some(mode) = current_mode.as_mut() {
        if is_repeat {
            return handle_lose_ends(Some(mode.clone()), &mut state, false);
        }
        if is_key_down && !is_repeat {
            state.time_pressed = current_time_ms() as u128;
            if mode.handle_key_down_event(&mut state) {
                return handle_lose_ends(Some(mode.clone()), &mut state, false);
            } else {
                return handle_lose_ends(Some(mode.clone()), &mut state, true);
            }
        } else if is_key_up {
            if mode.check_if_deactivates(&mut state) {
                let elapsed_millis = current_time_ms().abs_diff(state.time_pressed);
                info!("Elapsed time since key down: {}ms", elapsed_millis);
                if elapsed_millis < 200 {
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
            return handle_lose_ends(Some(mode.clone()), &mut state, true);
        }
    } else {
        if is_key_down && !is_repeat {
            for mode in AVAILABLE_MODES.lock().unwrap().iter_mut() {
                if mode.get_activation_keys().contains(&vk_code) {
                    mode.set_activated_by(vk_code);
                    state.time_pressed = current_time_ms();
                    info!("Detected a key down, it matches an activation key. Setting current mode to {}", mode.get_name());
                    return handle_lose_ends(Some(mode.clone()), &mut state, false);
                }
            }
        }
    }
    return handle_lose_ends(current_mode.clone(), &mut state, true);
}

fn main() {
    println!(
        "Starting Bushido Keys version {}\n",
        env!("CARGO_PKG_VERSION")
    );
    let title = r#"
            )              
            (             )    (          ( /(              
        ( )\   (     ( /((   )\ )       )\()) (  (        
        )((_) ))\ (  )\())\ (()/( (   |((_)\ ))\ )\ ) (   
        ((_)_ /((_))\((_)((_) ((_)))\  |_ ((_)((_|()/( )\  
        | _ |_))(((_) |(_|_) _| |((_) | |/ (_))  )(_)|(_) 
        | _ \ || (_-< ' \| / _` / _ \   ' </ -_)| || (_-< 
        |___/\_,_/__/_||_|_\__,_\___/  _|\_\___| \_, /__/ 
                                                |__/     
    "#;
    println!("\x1b[38;5;208m{}\x1b[0m", title);

    let exe_path = env::current_exe().expect("Failed to get current executable path");
    // Initialize logger with environment variables (RUST_LOG=debug, info, warn, error)
    env_logger::init();
    // get $env:USERPROFILE
    let home_dir = env::var("USERPROFILE").expect("Failed to get home directory");
    // get home directory
    let bushido_config_dir = Path::new(&home_dir).join(".bushido_keys_config");
    println!(
        "Configuration files can be found in: {:?}",
        bushido_config_dir
    );

    if !bushido_config_dir.exists() {
        println!("{:?} does not exist", bushido_config_dir);
        fs::create_dir(&bushido_config_dir).expect("Failed to create config directory");
        if bushido_config_dir.exists() {
            println!("Success! {:?} now exists", bushido_config_dir);
        }
    }
    // if config_dir modes.json does not exist, create it
    let modes_json_dir = bushido_config_dir.join("modes.json");
    let modes_config: ModesConfig;
    let json_str: String;
    if modes_json_dir.exists() {
        println!("modes_json file exists and is at {:?}", modes_json_dir);
        // load modes_config from file
        json_str = fs::read_to_string(&modes_json_dir).expect("Failed to read modes.json");
        modes_config = serde_json::from_str(&json_str).expect("Failed to parse modes.json");
        println!("Loaded modes_config from {:?}", modes_json_dir);
        debug!("modes_config: {:?}", modes_config);
    } else {
        println!("modes.json config file does not exist, creating it");
        // create it
        let default_modes_config: ModesConfig = serde_json::from_str(&mode_json::get_json_str())
            .expect("Failed to parse default modes config");
        modes_config = default_modes_config.clone();
        json_str = serde_json::to_string_pretty(&default_modes_config)
            .expect("Failed to serialize default modes config");
        println!("Serializing modes to modes.json file to make sure everything matches.");
        fs::write(&modes_json_dir, &json_str)
            .expect("Failed to write default modes config to file");
        println!("Successfully wrote to {:?} ", modes_json_dir);
    }
    // Load available modes from configuration.
    // let config_str = fs::read_to_string("config/modes.json")
    //     .expect("Failed to read modes.json in the working directory.");

    let mut available_modes: Vec<Box<dyn Mode + Send>> = Vec::new();
    for mode_cfg in modes_config.modes {
        // For demonstration, create a BasicMode from each configuration.
        let mode_instance = BasicMode::new(mode_cfg);
        info!("Loaded mode: {}", mode_instance.config.get_name());
        info!("Activation keys: {:?}", mode_instance.get_activation_keys());
        available_modes.push(Box::new(mode_instance));
    }
    info!("Loaded modes config about to add mouse mode");
    let mouse_config_path = bushido_config_dir.join("mouse_config.json");
    available_modes.push(Box::new(MouseMode::new(&mouse_config_path)));
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
