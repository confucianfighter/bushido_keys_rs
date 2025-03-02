// mode.rs
use crate::key_and_modifiers::KeyAndModifiers;
use crate::key_state::KeyState;
use std::collections::HashMap;

pub trait Mode: Send {
    fn handle_key_down_event(&mut self, key_state: &mut KeyState) -> bool;
    fn handle_key_up_event(&mut self, key_state: &mut KeyState) -> bool;
    fn update(&mut self);
    fn get_name(&self) -> &str;
    fn get_activation_keys(&self) -> &Vec<u32>;
    /// Check if a key-up event should deactivate this mode.
    fn check_if_deactivates<'a, 'b>(&'a mut self, key_state: &'b mut KeyState) -> bool;

    /// Clone this mode as a boxed trait object.
    fn clone_box(&self) -> Box<dyn Mode + Send>;
    fn set_activated_by(&mut self, key_code: u32);
    fn get_activated_by(&self) -> Option<u32>;
}

impl Clone for Box<dyn Mode + Send> {
    fn clone(&self) -> Box<dyn Mode + Send> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct BasicMode {
    pub name: String,
    pub key_mapping: HashMap<u32, KeyAndModifiers>,
    pub activation_keys: Vec<u32>,
    pub key_code_activated_by: Option<u32>,
}
