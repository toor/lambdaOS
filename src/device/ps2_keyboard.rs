use device::ps2_8042;
use device::keyboard;
use alloc::Vec;
use alloc::string::{String, ToString};

#[derive(Debug)]
struct KeyPair {
    left: bool,
    right: bool,
}

impl KeyPair {
    const fn new() -> Self {
        KeyPair { left: false, right: false}
    }

    fn is_pressed(&self) -> bool {
        self.left || self.right
    }
}

pub enum Modifiers {
    AltLeft(bool),
    AltRight(bool),
    CapsLock,
    ControlLeft(bool),
    ControlRight(bool),
    NumLock,
    ScrollLock,
    ShiftLeft(bool),
    ShiftRight(bool),
}

struct ModifierState {
    shift: KeyPair,
    control: KeyPair,
    alt: KeyPair,
    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,
}

impl ModifierState {
    const fn new() -> Self {
        ModifierState {
            shift: KeyPair::new(),
            control: KeyPair::new(),
            alt: KeyPair::new(),
            caps_lock: false,
            num_lock: false,
            scroll_lock: false,
        }
    }

    fn use_uppercase_letters(&self) -> bool {
        self.shift.is_pressed() ^ self.caps_lock
    }
    
    /// Apply modifiers to ascii and return updated ascii.
    fn apply_to(&self, ascii: char) -> String {
        if self.use_uppercase_letters() {
            ascii.to_uppercase().collect()
        } else {
            ascii.to_string()
        }
    }
    
    /// Update modifier state.
    fn update(&mut self, modifier: Modifiers) {
        use self::Modifiers::*;

        match modifier {
            AltLeft(m) => self.alt.left = m,
            AltRight(m) => self.alt.right = m,
            CapsLock => self.caps_lock = !self.caps_lock,
            ControlLeft(m) => self.control.left = m,
            ControlRight(m) => self.control.right = m,
            NumLock => self.num_lock = !self.num_lock,
            ScrollLock => self.num_lock = !self.scroll_lock,
            ShiftLeft(m) => self.shift.left = m,
            ShiftRight(m) => self.shift.right = m,
        }
    }
}

/// Possible types of keyboard input we might receive,
pub enum Key {
    Ascii(u8),
    Meta(Modifiers),
    LowerAscii(u8),
}

pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

pub fn parse_key(scancode: u8) {
    let sequence: u64 = retrieve_bytes(scancode);

    if let Some(key) = keyboard::get_key(sequence) {
        match key {
            //TODO.
        }
    }
}

/// Read bytes until end of sequence and combine into a number.
fn retrieve_bytes(scancode: u8) -> u64 {
    let mut byte_sequence: Vec<u8> = vec![scancode];

    // These scancodes are special - they indicate the start of a byte sequence which is sent when
    // some keys are pressed. If they are the byte we receive, read until the end of the sequence.
    if scancode == 0xE0 || scancode == 0xE1 {
        // Read another byte from the keyboard.
        let check: u8 = ps2_8042::read_char();

        if let Some(byte) = keyboard::is_special_key(check) {
            byte_sequence.push(byte);
        }
    }


    byte_sequence
        .iter()
        .rev()
        .fold(0, |acc, &b| (acc << 1) + b as u64)
}
