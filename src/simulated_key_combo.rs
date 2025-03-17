#[derive(Debug, Clone)]
pub struct SimulatedKeyCombo {
    pub key_code: u32,
    // give modifiers a length of 4
    pub modifiers: [u32; 4],
}
