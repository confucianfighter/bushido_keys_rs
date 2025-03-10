use crate::event::Event;
use std::mem::MaybeUninit;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

/// Subscribes to keyboard events using a Windows low-level keyboard hook
pub fn subscribe_to_keyboard_events(
    handler: unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT,
) -> Result<HHOOK, String> {
    unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None)
            .expect("Failed to get module handle")
            .into();
        let hook_handle = SetWindowsHookExW(WH_KEYBOARD_LL, Some(handler), Some(h_instance), 0)
            .expect("Failed to install keyboard hook");
        if hook_handle.is_invalid() {
            eprintln!("Failed to install keyboard hook.");
            return Err("Failed to install keyboard hook.".to_string());
        }
        Ok(hook_handle)
    }
}

/// Starts the Windows message loop that processes keyboard events
pub fn start_message_loop(hook_handle: HHOOK) {
    unsafe {
        let mut msg = MaybeUninit::<MSG>::uninit();
        while GetMessageW(msg.as_mut_ptr(), None, 0, 0).into() {
            TranslateMessage(msg.as_ptr());
            DispatchMessageW(msg.as_ptr());
        }
        stop_message_loop(hook_handle);
    }
}

/// Stops the keyboard hook and cleans up resources
pub fn stop_message_loop(hook_handle: HHOOK) {
    unsafe {
        UnhookWindowsHookEx(hook_handle);
    }
}

/// Forwards a keyboard event to the next hook in the chain
pub fn forward_event(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}
