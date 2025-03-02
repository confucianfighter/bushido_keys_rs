// space_mode.rs
use crate::conversion::{string_to_modifier, string_to_vk};
use crate::input_simulator;
use crate::input_simulator::get_char_from_vk_code;
use crate::input_simulator::get_vk_code_from_char;
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

#[derive(Debug, Clone)]
pub struct MouseMode {
    pub config: ModeConfig,
    pub key_mapping: HashMap<u32, KeyAndModifiers>,
    pub activation_keys: Vec<u32>,
    pub key_code_activated_by: Option<u32>,
    mouse_vel_x: f64,
    mouse_vel_y: f64,
    acceleration: f64,
    friction: f64,
    max_speed: f64,
    w_down: bool,
    a_down: bool,
    s_down: bool,
    d_down: bool,
    i_down: bool,
    j_down: bool,
    k_down: bool,
    l_down: bool,
    fps: f64,
    last_update_millis: u128,
    wasd_acceleration: f64,
}

impl MouseMode {
    pub fn new() -> Self {
        let activation_keys = vec![get_vk_code_from_char(' ') as u32];
        let config = ModeConfig {
            name: "MouseMode".to_string(),
            activation_keys: vec![" ".to_string()],
            key_mapping: HashMap::new(),
        };
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
            config,
            key_mapping,
            activation_keys,
            key_code_activated_by: None,
            mouse_vel_x: 0.0,
            mouse_vel_y: 0.0,
            // pixels per second per second
            acceleration: 500.0,
            // pixels per second per second
            wasd_acceleration: 1500.0,
            friction: 0.999,
            max_speed: 2000.0,
            w_down: false,
            a_down: false,
            s_down: false,
            d_down: false,
            i_down: false,
            j_down: false,
            k_down: false,
            l_down: false,
            fps: 60.0,
            last_update_millis: current_time_ms(),
        }
    }
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
        let w_code = get_vk_code_from_char('W');
        let a_code = get_vk_code_from_char('A');
        let s_code = get_vk_code_from_char('S');
        let d_code = get_vk_code_from_char('D');
        let i_code = get_vk_code_from_char('I');
        let j_code = get_vk_code_from_char('J');
        // add detective emoji
        debug!("🕵️‍♂️🕵️‍♂️🕵️‍♂️ j_code is {}", j_code);
        debug!("🕵️‍♂️🕵️‍♂️🕵️‍♂️ key_state.vk_code is {}", key_state.vk_code);
        let k_code = get_vk_code_from_char('K');
        let l_code = get_vk_code_from_char('L');
        if key_state.vk_code == w_code as i32 {
            self.w_down = true;
            return true;
        } else if key_state.vk_code == a_code as i32 {
            self.a_down = true;
            return true;
        } else if key_state.vk_code == s_code as i32 {
            self.s_down = true;
            return true;
        } else if key_state.vk_code == d_code as i32 {
            self.d_down = true;
            return true;
        } else if key_state.vk_code == i_code as i32 {
            self.i_down = true;
            return true;
        } else if key_state.vk_code == j_code as i32 {
            self.j_down = true;
            return true;
        } else if key_state.vk_code == k_code as i32 {
            debug!(" kkkkkkkkkkkkkkk_down");
            self.k_down = true;
            return true;
        } else if key_state.vk_code == l_code as i32 {
            self.l_down = true;
            // log this with a really bright emoji
            debug!("🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥 l_down");
            return true;
        }
        match get_char_from_vk_code(key_state.vk_code as u32) {
            'Q' => {
                input_simulator::simulate_left_down();
                true
            }
            'E' => {
                input_simulator::simulate_right_down();
                true
            }
            'M' => {
                input_simulator::simulate_middle_down();
                true
            }

            _ => false,
        }
    }
    fn handle_key_up_event<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        let w_code = get_vk_code_from_char('W');
        let a_code = get_vk_code_from_char('A');
        let s_code = get_vk_code_from_char('S');
        let d_code = get_vk_code_from_char('D');
        let i_code = get_vk_code_from_char('I');
        let j_code = get_vk_code_from_char('J');
        let k_code = get_vk_code_from_char('K');
        let l_code = get_vk_code_from_char('L');
        if key_state.vk_code == w_code as i32 {
            self.w_down = false;
            return true;
        } else if key_state.vk_code == a_code as i32 {
            self.a_down = false;
            return true;
        } else if key_state.vk_code == s_code as i32 {
            self.s_down = false;
            return true;
        } else if key_state.vk_code == d_code as i32 {
            self.d_down = false;
            return true;
        } else if key_state.vk_code == i_code as i32 {
            self.i_down = false;
            return true;
        } else if key_state.vk_code == j_code as i32 {
            self.j_down = false;
            return true;
        } else if key_state.vk_code == k_code as i32 {
            self.k_down = false;
            return true;
        } else if key_state.vk_code == l_code as i32 {
            self.l_down = false;
            return true;
        }
        match get_char_from_vk_code(key_state.vk_code as u32) {
            'Q' => {
                input_simulator::simulate_left_up();
                true
            }
            'E' => {
                input_simulator::simulate_right_up();
                true
            }
            'M' => {
                input_simulator::simulate_middle_up();
                true
            }
            _ => false,
        }
    }
    fn update(&mut self) {
        let current_millis = current_time_ms();
        let delta_millis = current_millis.abs_diff(self.last_update_millis);
        let target_delta_millis = 1000.0 / self.fps;
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
        if self.a_down {
            self.mouse_vel_x -= self.wasd_acceleration * dt_seconds;
        }
        if self.d_down {
            self.mouse_vel_x += self.wasd_acceleration * dt_seconds;
        }
        if self.w_down {
            self.mouse_vel_y -= self.wasd_acceleration * dt_seconds;
        }
        if self.s_down {
            self.mouse_vel_y += self.wasd_acceleration * dt_seconds;
        }
        if self.i_down {
            self.mouse_vel_y -= self.acceleration * dt_seconds;
        }
        if self.j_down {
            self.mouse_vel_x -= self.acceleration * dt_seconds;
        }
        if self.k_down {
            self.mouse_vel_y += self.acceleration * dt_seconds;
        }
        if self.l_down {
            self.mouse_vel_x += self.acceleration * dt_seconds;
        }

        // self.mouse_vel_x *= self.friction;
        // self.mouse_vel_y *= self.friction;
        if self.mouse_vel_x.abs() > self.max_speed {
            self.mouse_vel_x = self.mouse_vel_x.signum() * self.max_speed;
        }
        if self.mouse_vel_y.abs() > self.max_speed {
            self.mouse_vel_y = self.mouse_vel_y.signum() * self.max_speed;
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
        self.config.get_name()
    }
    fn get_activation_keys(&self) -> &Vec<u32> {
        &self.activation_keys
    }
    fn check_if_deactivates<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        if let Some(key) = self.key_code_activated_by {
            if key == key_state.vk_code as u32 {
                info!(
                    "MouseMode ({}): deactivating because activator {:#X} was released",
                    self.config.get_name(),
                    key_state.vk_code
                );
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
