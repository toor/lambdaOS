use debug;

use constants;
use event::{EventType, IsListener};

use io::drivers::display::buffer;

#[derive(Clone, Copy)]
pub struct ControlKeyState {
    pub ctrl: bool,
    pub cmd: bool,
    pub alt: bool,
    pub shift: bool,
    pub caps_lock: bool,
    pub scroll_lock: bool,
    pub num_lock: bool,
}

#[derive(Clone, Copy)]
pub struct KeyEvent {
    pub event_type: EventType,
    pub scancode: u8,
    pub character: char,
    pub controls: ControlKeyState,
}

pub struct KeyEventScreenWriter {}

impl IsListener<KeyEvent> for KeyEventScreenWriter {
    fn handles_event(&self, ev: &KeyEvent) -> bool {
        !ev.controls.ctrl && !ev.controls.alt
    }

    fn notify(&self, ev: &KeyEvent) {
        if ev.scancode == constants::keyboard::ENTER_KEY.scancode {
            buffer::KEYBOARD_BUFFER.lock().new_line();
            return;
        }

        if ev.scancode == constants::keyboard::DELETE_KEY.scancode {
            buffer::KEYBOARD_BUFFER.lock().delete_byte();
            return;
        }

        if ev.character as u8 != 0 {
            buffer::KEYBOARD_BUFFER.lock().write_byte(ev.character as u8);
        }
    }
}

pub struct ToggleWatcher {}

impl IsListener<KeyEvent> for ToggleWatcher {
    fn handles_event(&self, ev: &KeyEvent) -> bool {
        ev.scancode == constants::keyboard::S_KEY.scancode && (ev.controls.ctrl || ev.controls.cmd);
    }

    #[allow(unused_variables)]
    fn notify(&self, ev: &KeyEvent) {
        //Switch the buffer
        buffer::toggle();
    }
}

pub struct DebugWatcher {}

impl IsListener<KeyEvent> for DebugWatcher {
    fn handles_event(&self, ev: &KeyEvent) -> bool {
        ev.scancode == constants::keyboard::D_KEY.scancode && (ev.controls.ctrl || ev.controls.cmd)
    }

    #[allow(unused_variables)]
    fn notify(&self, ev: &KeyEvent) {
        debug::debug();
    }
}

pub fn init() {
    use alloc::boxed::Box;
    use state;

    state::register_key_event_listener(Box::new(KeyEventScreenWriter {}));
    state::register_key_event_listener(Box::new(ToggleWatcher {}));
    state::register_key_event_listener(Box::new(DebugWatcher {}));
}