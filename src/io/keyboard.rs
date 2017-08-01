use spin::Mutex;

use io::Port;

use state;

use constants::keyboard::{Key, KEYS, PORT};

use event::{EventType, IsEvent};

use event::keyboard::{KeyEvent, ControlKeyState};

impl IsEvent for KeyEvent {
    fn event_type(&self) -> EventType {
        self.event_type
    }
}

impl KeyEvent {
    const fn new(scancode: u8, character: char, modifiers: &Modifiers) -> KeyEvent {
        KeyEvent {
            event_type: EventType::KeyEvent,
            scancode: scancode,
            character: character,
            controls: ControlKeyState {
                cmd: modifiers.l_cmd || modifiers.r_cmd,
                ctrl: modifiers.l_ctrl,
                alt: modifiers.l_alt || modifiers.r_alt,
                shift: modifiers.l_shift || modifiers.r_shift,
                caps_lock: modifiers.caps_lock,
                scroll_lock: false,
                num_lock: false,
            },
        }
    }
}

struct State {
    //Keyboard state. Includes the PS/2 serial port and information about currently pressed modifiers.
    port: Port<u8>, //nicer to wrap the port in a Port object
    modifiers: Modifiers,
}

#[allow(dead_code)]
struct Modifiers {
    l_ctrl: bool,
    l_shift: bool,
    r_shift: bool,
    caps_lock: bool,
    l_cmd: bool,
    r_cmd: bool,
    l_alt: bool,
    r_alt: bool,
    last_key: u8,
}

impl Modifiers {
    const fn new() -> Modifiers {
        Modifiers {
            l_ctrl: false,
            l_shift: false,
            r_shift: false,
            caps_lock: false,
            l_cmd: false,
            r_cmd: false,
            l_alt: false,
            r_alt: false,
            last_key: 0,
        }
    }

    #[allow(dead_code)]
    fn cmd(&self) -> bool {
        self.l_cmd || self.r_cmd
    }

    fn update(&mut self, scancode: u8) {

        // printk!("{:x} {:x}", self.last_key, scancode);

        match scancode {
            0x5B => self.l_cmd = true,
            0xDB => self.l_cmd = false,
            0x5C => self.r_cmd = true,
            0xDC => self.r_cmd = false,
            0x2A => self.l_shift = true,
            0xAA => self.l_shift = false,
            0x36 => self.r_shift = true,
            0xB6 => self.r_shift = false,
            0x1D => self.l_ctrl = true,
            0x9D => self.l_ctrl = false,
            0x3A => self.caps_lock = !self.caps_lock,
            _ => {}
        }

        self.last_key = scancode;
    }

    fn apply_to(&self, key: Key) -> Option<char> {

        // Only alphabetic keys honor caps lock, so first distinguish between
        // alphabetic and non alphabetic keys.
        if (0x10 <= key.scancode && key.scancode <= 0x19) ||
           (0x1E <= key.scancode && key.scancode <= 0x26) ||
           (0x2C <= key.scancode && key.scancode <= 0x32) {
            if (self.l_shift || self.r_shift) ^ self.caps_lock {
                return Some(key.upper);
            }
        } else {
            if self.l_shift || self.r_shift {
                return Some(key.upper);
            }
        }

        return Some(key.lower);
    }
}

/// Our global keyboard state, protected by a mutex.
static STATE: Mutex<State> = Mutex::new(State {
    port: unsafe { Port::new(PORT) },
    modifiers: Modifiers::new(),
});

/// Try to read a single input character
pub fn read() {

    let mut state = STATE.lock();

    // Read a single scancode off our keyboard port.
    let scancode: u8 = state.port.read();

    if scancode == 0xE0 {
        // Ignore
        return;
    }

    //Modifiers get first chance at the scancode
    state.modifiers.update(scancode);

    // We don't map any keys > 127.
    if scancode > 127 {
        return;
    }

    // Look up the ASCII keycode.
    if let Some(key) = KEYS[scancode as usize] {
        // The `as char` converts our ASCII data to Unicode, which is
        // correct as long as we're only using 7-bit ASCII.
        if let Some(transformed_ascii) = state.modifiers.apply_to(key) {
            state::dispatch_key_event(&KeyEvent::new(scancode,
                                                     transformed_ascii,
                                                     &state.modifiers));
            return;
        }
    }
}