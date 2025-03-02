// key_and_modifiers.rs
#[derive(Debug, Clone)]
pub struct KeyAndModifiers {
    pub key: u32,
    pub modifiers: Vec<u32>,
}

impl KeyAndModifiers {
    pub fn new(key: u32, modifiers: Vec<u32>) -> Self {
        Self { key, modifiers }
    }
}
