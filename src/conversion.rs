
pub fn string_to_vk(s: &str) -> u32 {
    match s {
        // Alphabet
        "A" => 0x41,
        "B" => 0x42,
        "C" => 0x43,
        "D" => 0x44,
        "E" => 0x45,
        "F" => 0x46,
        "G" => 0x47,
        "H" => 0x48,
        "I" => 0x49,
        "J" => 0x4A,
        "K" => 0x4B,
        "L" => 0x4C,
        "M" => 0x4D,
        "N" => 0x4E,
        "O" => 0x4F,
        "P" => 0x50,
        "Q" => 0x51,
        "R" => 0x52,
        "S" => 0x53,
        "T" => 0x54,
        "U" => 0x55,
        "V" => 0x56,
        "W" => 0x57,
        "X" => 0x58,
        "Y" => 0x59,
        "Z" => 0x5A,

        // Numbers
        "0" => 0x30,
        "1" => 0x31,
        "2" => 0x32,
        "3" => 0x33,
        "4" => 0x34,
        "5" => 0x35,
        "6" => 0x36,
        "7" => 0x37,
        "8" => 0x38,
        "9" => 0x39,

        // Function Keys
        "F1" => 0x70,
        "F2" => 0x71,
        "F3" => 0x72,
        "F4" => 0x73,
        "F5" => 0x74,
        "F6" => 0x75,
        "F7" => 0x76,
        "F8" => 0x77,
        "F9" => 0x78,
        "F10" => 0x79,
        "F11" => 0x7A,
        "F12" => 0x7B,

        // Numpad Keys
        "NUMPAD0" => 0x60,
        "NUMPAD1" => 0x61,
        "NUMPAD2" => 0x62,
        "NUMPAD3" => 0x63,
        "NUMPAD4" => 0x64,
        "NUMPAD5" => 0x65,
        "NUMPAD6" => 0x66,
        "NUMPAD7" => 0x67,
        "NUMPAD8" => 0x68,
        "NUMPAD9" => 0x69,
        "MULTIPLY" => 0x6A,
        "ADD" => 0x6B,
        "SEPARATOR" => 0x6C,
        "SUBTRACT" => 0x6D,
        "DECIMAL" => 0x6E,
        "DIVIDE" => 0x6F,

        // Special Characters
        ";" => 0xBA,  // VK_OEM_1
        ":" => 0xBA,  // Shift + VK_OEM_1
        "=" => 0xBB,  // VK_OEM_PLUS
        "+" => 0xBB,  // Shift + VK_OEM_PLUS
        "," => 0xBC,  // VK_OEM_COMMA
        "<" => 0xBC,  // Shift + VK_OEM_COMMA
        "-" => 0xBD,  // VK_OEM_MINUS
        "_" => 0xBD,  // Shift + VK_OEM_MINUS
        "." => 0xBE,  // VK_OEM_PERIOD
        ">" => 0xBE,  // Shift + VK_OEM_PERIOD
        "/" => 0xBF,  // VK_OEM_2
        "?" => 0xBF,  // Shift + VK_OEM_2
        "`" => 0xC0,  // VK_OEM_3
        "~" => 0xC0,  // Shift + VK_OEM_3
        "[" => 0xDB,  // VK_OEM_4
        "{" => 0xDB,  // Shift + VK_OEM_4
        "\\" => 0xDC, // VK_OEM_5
        "|" => 0xDC,  // Shift + VK_OEM_5
        "]" => 0xDD,  // VK_OEM_6
        "}" => 0xDD,  // Shift + VK_OEM_6
        "'" => 0xDE,  // VK_OEM_7
        "\"" => 0xDE, // Shift + VK_OEM_7

        // Control Keys
        "BACKSPACE" => 0x08,
        "TAB" => 0x09,
        "CLEAR" => 0x0C,
        "ENTER" => 0x0D,
        "SHIFT" => 0x10,
        "CTRL" => 0x11,
        "ALT" => 0x12,
        "PAUSE" => 0x13,
        "CAPSLOCK" => 0x14,
        "ESC" => 0x1B,
        "SPACEBAR" => 0x20,
        "PAGEUP" => 0x21,
        "PAGEDOWN" => 0x22,
        "END" => 0x23,
        "HOME" => 0x24,
        "LEFT" => 0x25,
        "UP" => 0x26,
        "RIGHT" => 0x27,
        "DOWN" => 0x28,
        "SELECT" => 0x29,
        "PRINT" => 0x2A,
        "EXECUTE" => 0x2B,
        "PRINTSCREEN" => 0x2C,
        "INSERT" => 0x2D,
        "DELETE" => 0x2E,
        "HELP" => 0x2F,

        // Windows Keys
        "LWIN" => 0x5B,
        "RWIN" => 0x5C,
        "WINDOWS" => 0x5B | 0x5C | 0x5D,
        "APPS" => 0x5D,

        // Numeric Keypad
        "NUMLOCK" => 0x90,
        "SCROLLLOCK" => 0x91,

        // Media Keys
        "VOLUME_MUTE" => 0xAD,
        "VOLUME_DOWN" => 0xAE,
        "VOLUME_UP" => 0xAF,

        _ => s.chars().next().unwrap() as u32,
    }
}

pub fn char_to_vk(s: char) -> u32 {
    //convert s to a string
    let s = s.to_string();
    //convert the string to a vk code
    //
    string_to_vk(&s)
}
pub fn vk_to_string(vk_code: u32) -> Option<String> {
    println!("vk_code: {}", vk_code);
    match vk_code {
        0x41 => Some("A".to_string()),
        0x42 => Some("B".to_string()),
        0x43 => Some("C".to_string()),
        0x44 => Some("D".to_string()),
        0x45 => Some("E".to_string()),
        0x46 => Some("F".to_string()),
        0x47 => Some("G".to_string()),
        0x48 => Some("H".to_string()),
        0x49 => Some("I".to_string()),
        0x4A => Some("J".to_string()),
        0x4B => Some("K".to_string()),
        0x4C => Some("L".to_string()),
        0x4D => Some("M".to_string()),
        0x4E => Some("N".to_string()),
        0x4F => Some("O".to_string()),
        0x50 => Some("P".to_string()),
        0x51 => Some("Q".to_string()),
        0x52 => Some("R".to_string()),
        0x53 => Some("S".to_string()),
        0x54 => Some("T".to_string()),
        0x55 => Some("U".to_string()),
        0x56 => Some("V".to_string()),
        0x57 => Some("W".to_string()),
        0x58 => Some("X".to_string()),
        0x59 => Some("Y".to_string()),
        0x5A => Some("Z".to_string()),
        0x60 => Some("`".to_string()),
        0x31 => Some("1".to_string()),
        0x32 => Some("2".to_string()),
        0x33 => Some("3".to_string()),
        0x34 => Some("4".to_string()),
        0x35 => Some("5".to_string()),
        0x36 => Some("6".to_string()),
        0x37 => Some("7".to_string()),
        0x38 => Some("8".to_string()),
        0x39 => Some("9".to_string()),
        0x40 => Some("0".to_string()),
        0xBD => Some("-".to_string()),
        0xBB => Some("=".to_string()),
        0xDB => Some("[".to_string()),
        0xDD => Some("]".to_string()),
        0xDC => Some("\\".to_string()),
        0xBA => Some(";".to_string()),
        0xDE => Some("'".to_string()),
        0x72 => Some("~".to_string()),
        0xBC => Some(",".to_string()),
        0xBE => Some(".".to_string()),
        0xBF => Some("/".to_string()),
        0xC0 => Some("`".to_string()),
        0x7C => Some("|".to_string()),
        // SHIFT
        0x10 | 0xA0 | 0xA1 => Some("SHIFT".to_string()),
        // CTRL
        0x11 | 0xA2 | 0xA3 => Some("CTRL".to_string()),
        // ALT
        0x12 | 0xA4 | 0xA5 => Some("ALT".to_string()),
        // WINDOWS
        0x5B | 0x5C | 0x5D => Some("WINDOWS".to_string()),
        // LWIN
        0x5B => Some("LWIN".to_string()),
        // RWIN
        0x5C => Some("RWIN".to_string()),
        // TAB
        0x09 => Some("TAB".to_string()),
        // ENTER
        0x0D => Some("ENTER".to_string()),
        // BACKSPACE
        0x08 => Some("BACKSPACE".to_string()),
        // ESCAPE
        0x1B => Some("ESCAPE".to_string()),
        // SPACE
        0x20 => Some("SPACE".to_string()),
        // DELETE
        0x2E => Some("DELETE".to_string()),
        // LEFT ARROW
        0x25 => Some("LEFT ARROW".to_string()),
        // RIGHT ARROW
        0x27 => Some("RIGHT ARROW".to_string()),
        // UP ARROW
        0x26 => Some("UP ARROW".to_string()),
        // DOWN ARROW
        0x28 => Some("DOWN ARROW".to_string()),
        // PAGE UP
        0x21 => Some("PAGE UP".to_string()),
        // PAGE DOWN
        0x22 => Some("PAGE DOWN".to_string()),
        // HOME
        0x24 => Some("HOME".to_string()),
        // END
        0x23 => Some("END".to_string()),
        // INSERT
        0x2D => Some("INSERT".to_string()),
        // NUM LOCK
        0x90 => Some("NUM LOCK".to_string()),
        // CAPS LOCK
        0x14 => Some("CAPS LOCK".to_string()),
        // SCROLL LOCK
        0x91 => Some("SCROLL LOCK".to_string()),
        // PRINT SCREEN
        0x2C => Some("PRINT SCREEN".to_string()),
        // PAUSE
        0x13 => Some("PAUSE".to_string()),
        // BREAK
        0x19 => Some("BREAK".to_string()),
        // PRINT SCREEN
        _ => None,
    }
    .inspect(|s| println!("That vk code translates to: the {:?} key.", s))
}

pub fn string_to_modifier(s: &str) -> u32 {
    // to uppercase
    // using let statements because I care more about the name than the type.
    let s = s.to_string();
    let s = s.to_uppercase();
    let s = s.trim();
    match s {
        "SHIFT" => 0x10,  // VK_SHIFT
        "CTRL" => 0x11,   // VK_CONTROL
        "ALT" => 0x12,    // VK_ALT
        "LSHIFT" => 0xA0, // VK_LSHIFT
        "RSHIFT" => 0xA1, // VK_RSHIFT
        "LCTRL" => 0xA2,  // VK_LCONTROL
        "RCTRL" => 0xA3,  // VK_RCONTROL
        "LALT" => 0xA4,   // VK_LALT
        "RALT" => 0xA5,
        "LWIN" => 0x5B,
        "RWIN" => 0x5C,
        "WINDOWS" => 0x5B | 0x5C | 0x5D,
        _ => 0,
    }
}
pub fn modifer_to_string_or_none(s: u32) -> Option<String> {
    match s {
        0x10 => Some("SHIFT".to_string()),
        0x11 => Some("CTRL".to_string()),
        0x12 => Some("ALT".to_string()),
        0xA0 => Some("LSHIFT".to_string()),
        0xA1 => Some("RSHIFT".to_string()),
        0xA2 => Some("LCTRL".to_string()),
        0xA3 => Some("RCTRL".to_string()),
        0xA4 => Some("LALT".to_string()),
        0xA5 => Some("RALT".to_string()),
        0x5B => Some("LWIN".to_string()),
        0x5C => Some("RWIN".to_string()),
        0x5D => Some("WINDOWS".to_string()),
        _ => None,
    }
}
