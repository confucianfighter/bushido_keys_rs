// src/input_simulator.rs
use log::info;
use windows::{core::*, Win32::Foundation::*, Win32::UI::Input::KeyboardAndMouse::*};

pub fn move_mouse(dx: i32, dy: i32) {
    // get screen width and height
    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_MOVE,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn simulate_left_down() {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}
// add simulate_left_up, simulate_right_down/up, simulate_middle_down/up.
pub fn simulate_left_up() {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTUP,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn simulate_right_down() {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn simulate_right_up() {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTUP,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn simulate_middle_down() {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_MIDDLEDOWN,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn simulate_middle_up() {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_MIDDLEUP,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn get_char_from_vk_code(vk_code: u32) -> char {
    let character = char::from_u32(vk_code as u32).unwrap();
    character
}



/// Simulates a key tap with optional modifier keys.
pub fn simulate_key_tap(vk_code: u32, modifiers: &[u32]) {
    // get char from vk_code
    let char = get_char_from_vk_code(vk_code);
    info!(
        "input_simulator.rs: top of simulating key tap function, main vk_code translates to: {}",
        char
    );
    let mut inputs = Vec::new();
    // Press modifier keys
    for &mod_vk in modifiers {
        let kb = KEYBDINPUT {
            wVk: VIRTUAL_KEY(mod_vk as u16),
            wScan: 0,
            dwFlags: KEYBD_EVENT_FLAGS(0),
            time: 0,
            dwExtraInfo: 0,
        };
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: kb },
        });
    }
    // Main key down
    let kb_down = KEYBDINPUT {
        wVk: VIRTUAL_KEY(vk_code as u16),
        wScan: 0,
        dwFlags: KEYBD_EVENT_FLAGS(0),
        time: 0,
        dwExtraInfo: 0,
    };
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: kb_down },
    });
    // Main key up
    let kb_up = KEYBDINPUT {
        wVk: VIRTUAL_KEY(vk_code as u16),
        wScan: 0,
        dwFlags: KEYEVENTF_KEYUP,
        time: 0,
        dwExtraInfo: 0,
    };
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: kb_up },
    });
    // Release modifier keys in reverse order
    for &mod_vk in modifiers.iter().rev() {
        let kb = KEYBDINPUT {
            wVk: VIRTUAL_KEY(mod_vk as u16),
            wScan: 0,
            dwFlags: KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: kb },
        });
    }
    unsafe {
        SendInput(&inputs, size_of::<INPUT>() as i32);
    }
}
