// basic_mode.rs
use crate::conversion::*;
use crate::input_simulator::simulate_key_tap;
use crate::key_and_modifiers::KeyAndModifiers;
use crate::key_state::KeyState;
use crate::mode::Mode;
use crate::mode_config::ModeConfig;
use crate::utils::current_time_ms;
use log::info;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BasicMode {
    pub config: ModeConfig,
    pub key_mapping: HashMap<u32, KeyAndModifiers>,
    pub activation_keys: Vec<u32>,
    /// Tracks the virtual key that activated this mode.
    pub activated_by: Option<u32>,
    pub auto_modifiers: Vec<u32>,
    pub was_mode_used: bool,
    pub was_repeat: bool,
}

impl BasicMode {
    pub fn new(config: ModeConfig) -> Self {
        let activation_keys = config
            .activation_keys
            .iter()
            .map(|s| string_to_vk(s))
            .collect();

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
        let auto_modifiers: Vec<u32> = config
            .auto_modifiers
            .iter()
            .map(|s| string_to_modifier(s))
            .collect();

        Self {
            config,
            key_mapping,
            activation_keys,
            auto_modifiers: auto_modifiers,
            activated_by: None,
            was_mode_used: false,
            was_repeat: false,
        }
    }
}

impl Mode for BasicMode {
    fn was_mode_used(&self) -> bool {
        self.was_mode_used
    }
    fn set_was_mode_used(&mut self, was_mode_used: bool) {
        self.was_mode_used = was_mode_used;
    }
    fn get_auto_modifiers(&self) -> &Vec<u32> {
        &self.auto_modifiers
    }
    fn was_repeat(&self) -> bool {
        self.was_repeat
    }
    fn set_was_repeat(&mut self, was_repeat: bool) {
        self.was_repeat = was_repeat;
    }
    fn handle_key_down_event<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        //println!("Mode used = {}", self.was_mode_used);
        // if the key is in the mapping or in the activation keys, then  true
        let mut handled = false;
        let vk_code = key_state.vk_code as u32;
        if self.activation_keys.contains(&vk_code) {
            handled = true;
        }
        if self.key_mapping.contains_key(&vk_code) {
            handled = true;
        }

        // If this key is one of the activation keys and we haven't activated yet,
        // record it as the activator.
        // Otherwise, if the key is in our key mapping, process it.
        // if let Some(mapping) = self.key_mapping.get(&vk_code) {
        //     info!(
        //         "BasicMode: determine that we need to remap key {:#X} to {:#X} with modifiers {:?}",
        //         key_state.vk_code, mapping.key, mapping.modifiers
        //     );
        //     // simulate ctrl+c if it's the c key

        //     // simulate the key tap
        //     simulate_key_tap(mapping.key, &mapping.modifiers, &self.get_auto_modifiers());
        // }
        handled = true;

        return handled;
    }
    fn handle_key_up_event<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        self.set_was_mode_used(true);
        let vk_code = key_state.vk_code as u32;
        let mut handled = false;
        if self.activation_keys.contains(&vk_code) {
            handled = true;
        }
        if self.key_mapping.contains_key(&vk_code) {
            handled = true;
        }
        if let Some(mapping) = self.key_mapping.get(&vk_code) {
            info!(
                "BasicMode: determine that we need to remap key {:#X} to {:#X} with modifiers {:?}",
                key_state.vk_code, mapping.key, mapping.modifiers
            );
            // simulate ctrl+c if it's the c key
            key_state.time_released = Instant::now();
            key_state.held = false;
            handled = true;

            // simulate the key tap
            // combine the modifiers with the auto modifiers
            let modifiers = [
                &mapping.modifiers.clone().into_iter().collect::<Vec<u32>>()[..],
                //if key_state.was_shift_held_on_key_down {
                if key_state.was_shift_held_on_key_down {
                    &[0x10]
                } else {
                    &[]
                }, //} else {
                   //    &[]
            ]
            .concat();
            // if the key was held for more than 500ms, do not simulate the key tap
            if key_state.time_pressed.elapsed().as_millis() < Duration::from_millis(500).as_millis()
            {
                simulate_key_tap(mapping.key, &modifiers, &self.get_auto_modifiers());
            }
        } else {
            // Calculate modifiers directly into a Vec
            let mut modifiers = Vec::new();
            if key_state.was_shift_held_on_key_down {
                modifiers.push(0x10);
            }

            handled = true;
            // key_state.held = false; // Let main.rs handle state update

            // Pass a slice reference (&modifiers) to simulate_key_tap
            if key_state.time_pressed.elapsed().as_millis() < Duration::from_millis(500).as_millis()
            {
                simulate_key_tap(vk_code, &modifiers, &self.get_auto_modifiers());
            }
        }
        // check if key is in key_mapping
        return handled;
    }
    fn update(&mut self) {
        // BasicMode has no periodic update.
    }
    fn get_name(&self) -> &str {
        self.config.get_name()
    }
    fn get_activation_keys(&self) -> &Vec<u32> {
        &self.activation_keys
    }
    fn check_if_deactivates<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool {
        // Only deactivate if the key released is the one that activated the mode.
        info!(
            "Checking if BasicMode ({}) deactivates",
            self.config.get_name()
        );
        if let Some(activator) = self.activated_by {
            info!("Activator is {:?}", activator);
            if activator == key_state.vk_code as u32 {
                info!(
                    "BasicMode ({}): deactivating because activator {:#X} was released",
                    self.config.get_name(),
                    key_state.vk_code
                );
                // testing my keys can I write one hundred? 100. Yes!
                self.activated_by = None;
                key_state.held = false;
                // check how long the key was held
                // subtract the time pressed from the time released in a non-overflowing way
                // let time_held = key_state
                //     .time_released
                //     .saturating_sub(key_state.time_pressed);
                // if time_held < 200 {
                //     // simulate a key tap
                //     simulate_key_tap(key_state.vk_code as u32, &[]);
                // }
                return true;
            }
        }
        false
    }
    fn clone_box(&self) -> Box<dyn Mode + Send> {
        Box::new(self.clone())
    }
    fn set_activated_by(&mut self, key_code: u32) {
        info!("Setting activated_by to {:?}", key_code);
        self.activated_by = Some(key_code);
    }
    fn get_activated_by(&self) -> Option<u32> {
        self.activated_by
    }
}
