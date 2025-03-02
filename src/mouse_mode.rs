// space_mode.rs
use crate::conversion::{string_to_modifier, string_to_vk};
use crate::input_simulator;
use crate::input_simulator::get_char_from_vk_code;
use crate::conversion::char_to_vk;
use crate::key_and_modifiers::KeyAndModifiers;
use crate::key_state::KeyState;
use crate::mode::Mode;
use crate::mode_config::ModeConfig;
use crate::utils::current_time_ms;
use log::debug;
use log::info;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
// use serde for mouse config struct
use serde::{Deserialize, Serialize};
use std::char;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MouseConfig {
    fast_up_key: char,
    fast_down_key: char,
    fast_left_key: char,
    fast_right_key: char,
    slow_up_key: char,
    slow_down_key: char,
    slow_left_key: char,
    slow_right_key: char,
    fast_acceleration: f64,
    slow_acceleration: f64,
    friction: f64,
    max_speed: f64,
    fps: f64,
    left_click_key: char,
    right_click_key: char,
    middle_click_key: char,
    dual_wield_multiplier: f64,
    activation_keys: Vec<String>,
}
// define default values for mouse config
impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            //add key for return to center of screen
            //add key to jump to next monitor
            //consider adding blocked keys to avoid the user setting CTRL,C, or Z to avoid user disabling the abilityu to interrupt the program, ignore these keys in the...
            //... json config and throw a meaningful error to the user
            // define custom char to 
            //add mouse scroll wheel actions
            fast_up_key: 'W',
            fast_down_key: 'S',
            fast_left_key: 'A',
            fast_right_key: 'D',
            slow_up_key: 'O',
            slow_down_key: 'L',
            slow_left_key: 'K',
            slow_right_key: ';',
            // pixels per second per second
            fast_acceleration: 1500.0,
            // pixels per second per second
            slow_acceleration: 500.0,
            friction: 0.999,
            max_speed: 2000.0,
            fps: 60.0,
            left_click_key: 'Q',
            right_click_key: 'E',
            middle_click_key: 'M',
            dual_wield_multiplier: 0.5,
            //setup ability to set multiple keys as the activation keys, search for ' ' and " ", replace with code to pull activation_keys from a list
            activation_keys: vec![" ".to_string()],
        }
    }
}
#[derive(Debug, Clone)]
pub struct MouseMode {
    pub config: MouseConfig,
    // ignore
    pub activation_keys: Vec<u32>,

    last_update_millis: u128,

    pub key_mapping: HashMap<u32, KeyAndModifiers>,

    pub key_code_activated_by: Option<u32>,

    mouse_vel_x: f64,

    mouse_vel_y: f64,

    fast_up_pressed: bool,
    fast_left_pressed: bool,
    fast_down_pressed: bool,
    fast_right_pressed: bool,
    slow_up_pressed: bool,
    slow_left_pressed: bool,
    slow_down_pressed: bool,
    slow_right_pressed: bool,
}

impl MouseMode {
    pub fn new() -> Self {
        let activation_keys = vec![char_to_vk(' ') as u32];
        let config = ModeConfig {
            name: "MouseMode".to_string(),
            activation_keys: vec![" ".to_string()],
            key_mapping: HashMap::new(),
        };
        let mouse_config = load_mouse_config();
        let key_mapping = config
            .key_mapping
            .iter()
            .map(|(src_key, entry)| {
                (
                    string_to_vk(src_key),
                    KeyAndModifiers {
                        key: string_to_vk(&entry.key),
                        modifiers: entry
                            .modifiers
                            .iter()
                            .map(|m| string_to_modifier(m))
                            .collect(),
                    },
                )
            })
            .collect();

        Self {
            config: mouse_config,
            key_mapping,
            activation_keys,
            key_code_activated_by: None,
            mouse_vel_x: 0.0,
            mouse_vel_y: 0.0,
            fast_up_pressed: false,
            fast_left_pressed: false,
            fast_down_pressed: false,
            fast_right_pressed: false,
            slow_up_pressed: false,
            slow_left_pressed: false,
            slow_down_pressed: false,
            slow_right_pressed: false,
            last_update_millis: current_time_ms(),
        }
    }
}

fn load_mouse_config() -> MouseConfig {
    let config_path = "config/mouse_config.json";
    let config_str = std::fs::read_to_string(config_path).unwrap();
    let config: MouseConfig = serde_json::from_str(&config_str).unwrap();
    // convert all keys in the config

    config
}
impl Mode for MouseMode {
    fn handle_key_down_event<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        let vk_code = key_state.vk_code as u32;
        // If this key is an activation key and not already activated, record it.
        if self.activation_keys.contains(&vk_code) && self.key_code_activated_by.is_none() {
            self.key_code_activated_by = Some(vk_code);
            info!("MouseMode activated by key {:#X}", key_state.vk_code);
            return true;
        }
        // convert key_state.vk_code to a char
        let key_char = char::from_u32(key_state.vk_code as u32).unwrap();
        debug!("🐭 mouse mode detected key_char: {}", key_char);
        // get all codes for w, a, s, d, i, j, k, l
        let fast_up_code = char_to_vk(self.config.fast_up_key);
        let fast_left_code = char_to_vk(self.config.fast_left_key);
        let fast_down_code = char_to_vk(self.config.fast_down_key);
        let fast_right_code = char_to_vk(self.config.fast_right_key);
        let slow_up_code = char_to_vk(self.config.slow_up_key);
        let slow_left_code = char_to_vk(self.config.slow_left_key);
        let slow_down_code = char_to_vk(self.config.slow_down_key);
        let slow_right_code = char_to_vk(self.config.slow_right_key);
        
        if key_state.vk_code == fast_up_code as i32 {
            self.fast_up_pressed = true;
            return true;
        } else if key_state.vk_code == fast_left_code as i32 {
            self.fast_left_pressed = true;
            return true;
        } else if key_state.vk_code == fast_down_code as i32 {
            self.fast_down_pressed = true;
            return true;
        } else if key_state.vk_code == fast_right_code as i32 {
            self.fast_right_pressed = true;
            return true;
        } else if key_state.vk_code == slow_up_code as i32 {
            self.slow_up_pressed = true;
            return true;
        } else if key_state.vk_code == slow_left_code as i32 {
            self.slow_left_pressed = true;
            return true;
        } else if key_state.vk_code == slow_down_code as i32 {
            
            self.slow_down_pressed = true;
            return true;
        } else if key_state.vk_code == slow_right_code as i32 {
            self.slow_right_pressed = true;
            // log this with a really bright emoji
            debug!("🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥 slow_right_down");
            return true;
        }
        let left_click = self.config.left_click_key;
        let right_click = self.config.right_click_key;

        let middle_click = self.config.middle_click_key;
        let key_pressed = get_char_from_vk_code(vk_code);
        match get_char_from_vk_code(key_state.vk_code as u32) {
            c if c == left_click => {
                input_simulator::simulate_left_down();
                true
            },
            c if c == right_click => {
                input_simulator::simulate_right_down();
                true
            },
            c if c == middle_click => {
                input_simulator::simulate_middle_down();
                true
            }

            _ => false,
        }
    }
    fn handle_key_up_event<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        let fast_up_code = char_to_vk(self.config.fast_up_key);
        let fast_left_code = char_to_vk(self.config.fast_left_key);
        let fast_down_code = char_to_vk(self.config.fast_down_key);
        let fast_right_code = char_to_vk(self.config.fast_right_key);
        let slow_up_code = char_to_vk(self.config.slow_up_key);
        let slow_left_code = char_to_vk(self.config.slow_left_key);
        let slow_down_code = char_to_vk(self.config.slow_down_key);
        let slow_right_code = char_to_vk(self.config.slow_right_key);
        let left_click_code = char_to_vk(self.config.left_click_key);
        let right_click_code = char_to_vk(self.config.right_click_key);
        let middle_click_code = char_to_vk(self.config.middle_click_key);
        
        if key_state.vk_code == fast_up_code as i32 {
            self.fast_up_pressed = false;
            return true;
        } else if key_state.vk_code == fast_left_code as i32 {
            self.fast_left_pressed = false;
            return true;
        } else if key_state.vk_code == fast_down_code as i32 {
            self.fast_down_pressed = false;
            return true;
        } else if key_state.vk_code == fast_right_code as i32 {
            self.fast_right_pressed = false;
            return true;
        } else if key_state.vk_code == slow_up_code as i32 {
            self.slow_up_pressed = false;
            return true;
        } else if key_state.vk_code == slow_left_code as i32 {
            self.slow_left_pressed = false;
            return true;
        } else if key_state.vk_code == slow_down_code as i32 {
            self.slow_down_pressed = false;
            return true;
        } else if key_state.vk_code == slow_right_code as i32 {
            self.slow_right_pressed = false;
            return true;
        }
        let _left_click = self.config.left_click_key;
        let _right_click = self.config.right_click_key;
        let _middle_click = self.config.middle_click_key;
        match get_char_from_vk_code(key_state.vk_code as u32) {
            c if c == _left_click => {
                input_simulator::simulate_left_up();
                true
            }
            c if c == _right_click => {
                input_simulator::simulate_right_up();
                true
            }
            c if c == _middle_click => {
                input_simulator::simulate_middle_up();
                true
            }
            _ => false,
        }
    }
    fn update(&mut self) {
        let current_millis = current_time_ms();
        let delta_millis = current_millis.abs_diff(self.last_update_millis);
        let target_delta_millis = 1000.0 / self.config.fps;
        // add  clock emoji in the debug message
        debug!(
            "🐭>>>>>>>>>>>>>>>>>>>>> delta_millis: {}, target_delta_millis: {}",
            delta_millis, target_delta_millis
        );
        let target_delta_millis = target_delta_millis as u128;
        debug!(
            "🐭 target_delta_millis after conversion: {}",
            target_delta_millis
        );
        if delta_millis < target_delta_millis {
            // add ninja emoji in the debug message
            debug!("[insert ninja here] Not updating mouse mode because delta_millis {} < target_delta_millis {}\x1b[0m", delta_millis, target_delta_millis);
            return;
        } else {
            debug!(
                "Just detected that delta_millis {} >= target_delta_millis {}",
                delta_millis, target_delta_millis
            );

            self.last_update_millis = current_time_ms();
        }
        // todo: compute motion by our dt
        debug!("🐭 Updating mouse mode");
        // check all 8 directions
        let dt_seconds = delta_millis as f64 / 1000.0;
        if self.fast_left_pressed {
            self.mouse_vel_x -= self.config.fast_acceleration * dt_seconds;
        }
        if self.fast_right_pressed {
            self.mouse_vel_x += self.config.fast_acceleration * dt_seconds;
        }
        if self.fast_up_pressed {
            self.mouse_vel_y -= self.config.fast_acceleration * dt_seconds;
        }
        if self.fast_down_pressed {
            self.mouse_vel_y += self.config.fast_acceleration * dt_seconds;
        }
        if self.slow_left_pressed {
            self.mouse_vel_x -= self.config.slow_acceleration * dt_seconds;
        }
        if self.slow_right_pressed {
            self.mouse_vel_x += self.config.slow_acceleration * dt_seconds;
        }
        if self.slow_up_pressed {
            self.mouse_vel_y -= self.config.slow_acceleration * dt_seconds;
        }
        if self.slow_down_pressed {
            self.mouse_vel_y += self.config.slow_acceleration * dt_seconds;
        }

        self.mouse_vel_x *= self.config.friction;
        self.mouse_vel_y *= self.config.friction;
        if self.mouse_vel_x.abs() > self.config.max_speed {
            self.mouse_vel_x = self.mouse_vel_x.signum() * self.config.max_speed;
        }
        if self.mouse_vel_y.abs() > self.config.max_speed {
            self.mouse_vel_y = self.mouse_vel_y.signum() * self.config.max_speed;
        }
        if self.mouse_vel_x.abs() >= 1.0 || self.mouse_vel_y.abs() >= 1.0 {
            // add mouse emoji in the info message
            debug!(
                "🐭🐭🐭🐭🐭🐭🐭🐭🐭🐭🐭🐭🐭 Moving mouse by {} {}",
                self.mouse_vel_x, self.mouse_vel_y
            );
            let x_move = self.mouse_vel_x * dt_seconds as f64;
            let y_move = self.mouse_vel_y * dt_seconds as f64;
            input_simulator::move_mouse(x_move as i32, y_move as i32);
        }
        // todo: use a dt for consistent speed.
        thread::sleep(Duration::from_millis(50));
    }
    fn get_name(&self) -> &str {
        "MouseMode"
    }
    fn get_activation_keys(&self) -> &Vec<u32> {
        &self.activation_keys
    }
    fn check_if_deactivates<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        if let Some(key) = self.key_code_activated_by {
            if key == key_state.vk_code as u32 {
                info!("MouseMode deactivated by key {:#X}", key_state.vk_code);
                self.key_code_activated_by = None;
                return true;
            }
        }
        false
    }

    fn clone_box(&self) -> Box<dyn Mode + Send> {
        Box::new(self.clone())
    }
    fn set_activated_by(&mut self, key_code: u32) {
        self.key_code_activated_by = Some(key_code);
    }
    fn get_activated_by(&self) -> Option<u32> {
        self.key_code_activated_by
    }
}
