use crate::key_and_modifiers::KeyAndModifiers;
use crate::key_state::KeyState;
use crate::mode::ModeVariant;
use crate::mode_config::ModeConfig;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegularMode {
    config: ModeConfig,
    key_mapping: HashMap<u32, KeyAndModifiers>,
    activation_keys: Vec<u32>,
    key_code_activated_by: Option<u32>,
}
impl ModeVariant for RegularMode {
    fn handle_key_down_event(&mut self, key_state: &mut KeyState) -> bool {
        false
    }
    fn handle_key_up_event(&mut self, key_state: &mut KeyState) -> bool {
        false
    }
    fn update(&mut self) {}
    fn get_name(&self) -> &str {
        "Regular"
    }
    fn get_activation_keys(&self) -> &Vec<u32> {
        &self.activation_keys
    }
    fn check_if_deactivates(&mut self, key_state: &mut KeyState) -> bool {
        false
    }
    fn clone_box(&self) -> Box<dyn ModeVariant + Send> {
        Box::new(self.clone())
    }
    fn set_activated_by(&mut self, key_code: u32) {
        self.key_code_activated_by = Some(key_code);
    }
    fn get_activated_by(&self) -> Option<u32> {
        self.key_code_activated_by
    }
}
impl RegularMode {
    pub fn new() -> Self {
        Self {
            config: ModeConfig::default(),
            key_mapping: HashMap::new(),
            activation_keys: Vec::new(),
            key_code_activated_by: None,
        }
    }
}
// regular mode may or may not be any different from a keymapping mode. In fact might want to have an inner keymapping mode that is used by the regular mode.
