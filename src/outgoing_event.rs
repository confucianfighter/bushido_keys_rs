use crate::hook_manager::KeyboardEventHandler;
use crate::input_simulator::simulate_key_tap;
use crate::simulated_key_combo::SimulatedKeyCombo;
use crate::test_utils::init_test_logger;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use deref
use std::ops::Deref;
use std::sync::Arc;
// use send? and its not std::sync::Send
use log::debug;
use log::info;
use std::sync::atomic::AtomicBool;

lazy_static! {
    static ref OUTGOING_EVENTS: Mutex<OutgoingEventQueue> = Mutex::new(OutgoingEventQueue::new());
}

#[derive(Debug, Clone)]
pub struct OutgoingEvent {
    activating_key_code: u32,
    normal_mode_combo: SimulatedKeyCombo,
    remapping_combo: SimulatedKeyCombo,
    ready_to_send: bool,
    key_down: bool,
    is_normal_mapping: bool,
    time: Instant,
    cancel: bool,
    // give mod name a max length of 10
    mode_name: String,
}
impl OutgoingEvent {
    pub fn new(activating_key_code: u32, key_down: bool) -> Self {
        Self {
            activating_key_code,
            normal_mode_combo: SimulatedKeyCombo {
                key_code: activating_key_code,
                modifiers: [0; 4],
            },
            remapping_combo: SimulatedKeyCombo {
                key_code: activating_key_code,
                modifiers: [0; 4],
            },
            ready_to_send: false,
            key_down,
            is_normal_mapping: true,
            time: Instant::now(),
            cancel: false,
            mode_name: "Normal".to_string(),
        }
    }
    pub fn set_cancel(&mut self, cancel: bool) {
        self.cancel = cancel;
    }
    pub fn get_cancel(&self) -> bool {
        self.cancel
    }
    pub fn set_normal_mode_combo(&mut self, combo: SimulatedKeyCombo) {
        self.normal_mode_combo = combo;
    }
    pub fn get_normal_mode_combo(&self) -> &SimulatedKeyCombo {
        &self.normal_mode_combo
    }
    pub fn set_remapping_combo(&mut self, combo: SimulatedKeyCombo) {
        self.remapping_combo = combo;
    }
    pub fn get_remapping_combo(&self) -> &SimulatedKeyCombo {
        &self.remapping_combo
    }
    pub fn set_is_normal_mapping(&mut self, is_normal_mapping: bool) {
        self.is_normal_mapping = is_normal_mapping;
    }
    pub fn get_is_normal_mapping(&self) -> bool {
        self.is_normal_mapping
    }
    pub fn get_time(&self) -> Instant {
        self.time
    }
    pub fn set_mode_name(&mut self, mode_name: String) {
        self.mode_name = mode_name;
    }
    pub fn get_mode_name(&self) -> &String {
        &self.mode_name
    }
    pub fn get_activating_key_code(&self) -> u32 {
        self.activating_key_code
    }
    pub fn get_key_down(&self) -> bool {
        self.key_down
    }
    pub fn get_ready_to_send(&self) -> bool {
        self.ready_to_send
    }
    pub fn set_ready_to_send(&mut self, ready_to_send: bool) {
        self.ready_to_send = ready_to_send;
    }
}

#[derive(Debug, Clone)]
struct OutgoingEventQueue(Vec<OutgoingEvent>);

impl OutgoingEventQueue {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn get_latest_match_by_key_code(
        &self,
        key_code: u32,
        key_down: bool,
    ) -> Option<OutgoingEvent> {
        let mut latest_match: Option<OutgoingEvent> = None;
        self.0
            .iter()
            .find(|event| event.activating_key_code == key_code && event.key_down == key_down)
            .map(|e| e.clone());
        latest_match
    }
    pub fn push(&mut self, event: OutgoingEvent) {
        self.0.push(event);
    }
    pub fn remove(&mut self, event: OutgoingEvent) {
        // make sure there is a match on key_code and time
        let index = self
            .0
            .iter()
            .position(|e| {
                e.activating_key_code == event.activating_key_code && e.time == event.time
            })
            .unwrap();
        self.0.remove(index);
    }
    pub fn oldest(&self) -> Option<&OutgoingEvent> {
        self.0.first()
    }
}
// test mod
#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::IncomingEvent;
    use crate::hook_manager;
    use crate::input_simulator::simulate_key_tap;
    use crate::key_state::KeyState;
    use scopeguard::defer;
    use std::collections::HashMap;

    #[test]
    fn test_outgoing_event() {
        //init_test_logger();
        // set up hook manager
        // set up outgoing event queue
        // assign key down and key up handlers to hook manager

        hook_manager::subscribe_to_key_down(key_down_handler);
        hook_manager::subscribe_to_key_up(key_up_handler);
        // start outgoing event queue
        hook_manager::init_keyboard_hook();

        // start event processing loop in a new thread
        process_outgoing_queue();

        hook_manager::start_message_loop();
    }
    fn key_down_handler(
        key_state: &mut KeyState,
        incoming_event: &mut IncomingEvent,
        event_queue: &mut Vec<OutgoingEvent>,
        key_states: &mut HashMap<i32, Arc<Mutex<KeyState>>>,
        is_repeat: bool,
    ) -> bool {
        info!("-> key_down_handler: {:x}", key_state.vk_code);
        if key_state.vk_code == 0x11 || key_state.vk_code == 0x43 {
            return false;
        }
        let mut outgoing_event_queue: OutgoingEventQueue = OUTGOING_EVENTS.lock().unwrap().clone();
        if (is_repeat) {
            return true;
        } else {
            let mut event = OutgoingEvent::new(key_state.vk_code as u32, true);
            let sim_key_combo = SimulatedKeyCombo {
                key_code: key_state.vk_code as u32,
                modifiers: [0; 4],
            };
            event.set_normal_mode_combo(sim_key_combo.clone());
            event.set_is_normal_mapping(true);
            event.set_mode_name("Normal".to_string());
            event.set_remapping_combo(sim_key_combo.clone());

            outgoing_event_queue.push(event.clone());
            *OUTGOING_EVENTS.lock().unwrap() = outgoing_event_queue.clone();
        }
        return true;
    }
    fn key_up_handler(
        key_state: &mut KeyState,
        event: &mut IncomingEvent,
        event_queue: &mut Vec<OutgoingEvent>,
        key_states: &mut HashMap<i32, Arc<Mutex<KeyState>>>,
        is_repeat: bool,
    ) -> bool {
        // print the character of the key_state
        info!("-> key_up_handler: {:x}", key_state.vk_code);
        // if its the control key or the c key, return false
        if key_state.vk_code == 0x11 || key_state.vk_code == 0x43 {
            return false;
        }
        let mut outgoing_event_queue = OUTGOING_EVENTS.lock().unwrap().clone();

        // retrieve the latest outgoing event matching key_down = true and key_code = key_state.key_code
        let latest_match =
            outgoing_event_queue.get_latest_match_by_key_code(key_state.vk_code as u32, true);

        if let Some(mut e) = latest_match {
            debug!("-> found a keydown match for the key down event;");
            e.set_ready_to_send(true);
            outgoing_event_queue.remove(e.clone());
            outgoing_event_queue.push(e);
        }
        *OUTGOING_EVENTS.lock().unwrap() = outgoing_event_queue;
        true
    }

    fn process_outgoing_queue() {
        info!("🐱‍🚀started outgoing event queue thread.");
        // launch as thread
        let outgoing_event_queue_thread = std::thread::spawn(move || {
            loop {
                let mut outgoing_event_queue = OUTGOING_EVENTS.lock().unwrap().clone();

                // retrieve first in first out style
                // peek at oldest event, if cancel is true, remove it, if ready_to_send is true, send it
                while let Some(mut e) = outgoing_event_queue.oldest() {
                    info!("-> peeking at event: {:?}", e);
                    if e.get_cancel() {
                        outgoing_event_queue.remove(e.clone());
                    } else if e.get_ready_to_send() {
                        info!("-> simulating key tap");
                        simulate_key_tap(
                            e.get_normal_mode_combo().key_code,
                            &e.get_normal_mode_combo().modifiers,
                        );
                        outgoing_event_queue.remove(e.clone());
                    } else if e.get_time().elapsed() > Duration::from_secs(30) {
                        outgoing_event_queue.clear();
                        break;
                    // if it's not ready to send, let the thread relax.
                    } else if !e.get_ready_to_send() {
                        break;
                    }
                }
                defer! {
                    *OUTGOING_EVENTS.lock().unwrap() = outgoing_event_queue;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        });
    }
}
