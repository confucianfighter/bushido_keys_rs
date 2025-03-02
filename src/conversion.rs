// conversion.rs
pub fn string_to_vk(s: &str) -> u32 {
    match s {
        "A" => 0x41,
        "S" => 0x53,
        "D" => 0x44,
        "F" => 0x46,
        "G" => 0x47,
        "H" => 0x48,
        "J" => 0x4A,
        "K" => 0x4B,
        "L" => 0x4C,
        ";" => 0xBA, // typical VK code for semicolon (US keyboards)
        "1" => 0x31,
        "2" => 0x32,
        "3" => 0x33,
        "4" => 0x34,
        "5" => 0x35,
        "6" => 0x36,
        "7" => 0x37,
        "8" => 0x38,
        "9" => 0x39,
        "0" => 0x30,
        "!" => 0x31, // note: requires SHIFT modifier
        "@" => 0x32,
        "#" => 0x33,
        "$" => 0x34,
        "%" => 0x35,
        "^" => 0x36,
        "&" => 0x37,
        "*" => 0x38,
        "(" => 0x39,
        ")" => 0x30,
        _ => 0,
    }
}

pub fn string_to_modifier(s: &str) -> u32 {
    match s {
        "SHIFT" => 0x10, // VK_SHIFT
        // Add more mappings like CTRL, ALT, etc. if needed.
        _ => 0,
    }
}
