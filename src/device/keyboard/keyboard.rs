use device::keyboard::ps2_keyboard::{Key, KeyEvent};
use device::keyboard::ps2_keyboard::Key::*;
use device::keyboard::ps2_keyboard::Modifiers::*;

macro_rules! key_press {
    ($x:expr) => (Some(KeyEvent::Pressed($x)))
}

macro_rules! key_release {
    ($x:expr) => (Some(KeyEvent::Released($x)))
}

/// Gets a key from a given keyboard event.
pub fn get_key(scancode: u64) -> Option<Key> {
    match get_key_event(scancode) {
        Some(KeyEvent::Pressed(key)) => Some(key),
        Some(KeyEvent::Released(key)) => Some(key),
        _ => None,
    }
}

/// Calls `match_scancode` and is then matched on itself to retrieve a `Key` based on the returned
/// `KeyEvent`.
pub fn get_key_event(scancode: u64) -> Option<KeyEvent> {
    match_scancode(scancode)
} 

/// Special keys that are part of a byte sequence.
pub fn is_special_key(byte: u8) -> Option<u8> {
    match byte {
        0x5B | 0xDB => Some(byte),
        0x1D | 0x9D => Some(byte),
        0x5C | 0xDC => Some(byte),
        0x38 | 0xB8 => Some(byte),
        0x5D | 0xDD => Some(byte),
        0x52 | 0xD2 => Some(byte),
        0x47 | 0x97 => Some(byte),
        0x49 | 0xC9 => Some(byte),
        0x53 | 0xD3 => Some(byte),
        0x4F | 0xCF => Some(byte),
        0x51 | 0xD1 => Some(byte),
        0x48 | 0xC8 => Some(byte),
        0x4B | 0xCB => Some(byte),
        0x50 | 0xD0 => Some(byte),
        0x4D | 0xCD => Some(byte),
        0x35 | 0xB5 => Some(byte),
        0x1C | 0x9C => Some(byte),
        _ => None,
    }
}

/// Use range matching to convert our passed scancode to some type of ASCII or to update modifiers,
/// and return a key-event based on whether this was a key press/release (only relevant for
/// modifiers).
fn match_scancode(scancode: u64) -> Option<KeyEvent> {
    let idx = scancode as usize;
    match scancode {
        // ASCII Keys by keyboard row
        0x02...0x0D => key_press!(LowerAscii(b"1234567890-="[idx - 0x02])),
        0x10...0x1B => key_press!(LowerAscii(b"qwertyuiop[]"[idx - 0x10])),
        0x1E...0x28 => key_press!(LowerAscii(b"asdfghjkl;'"[idx - 0x1E])),
        0x2C...0x35 => key_press!(LowerAscii(b"zxcvbnm,./"[idx - 0x2C])),
        0x29 => key_press!(LowerAscii((b'`'))),
        0x2B => key_press!(LowerAscii((b'\\'))),

        // Non-modifiable ASCII keys
        0x01 => key_press!(Ascii(0x1B)),  // escape
        0x0E => key_press!(Ascii(0x8)),   // backspace
        0x0F => key_press!(Ascii(b'\t')), // tab
        0x1C => key_press!(Ascii(b'\n')), // newline
        0x39 => key_press!(Ascii(b' ')),  // space

        // Meta keys
        0x1D => key_press!(Meta(ControlLeft(true))),
        0xE01D => key_press!(Meta(ControlRight(true))),
        0x2A => key_press!(Meta(ShiftLeft(true))),
        0x36 => key_press!(Meta(ShiftRight(true))),
        0x38 => key_press!(Meta(AltLeft(true))),
        0xE038 => key_press!(Meta(AltRight(false))),
        0x3A => key_press!(Meta(CapsLock)),
        0x45 => key_press!(Meta(NumLock)),
        0x46 => key_press!(Meta(ScrollLock)),

        0xAA => key_release!(Meta(ShiftLeft(false))),
        0xB6 => key_release!(Meta(ShiftRight(false))),
        0x9D => key_release!(Meta(ControlLeft(false))),
        0xE09D => key_release!(Meta(ControlRight(false))),
        0xB8 => key_release!(Meta(AltLeft(false))),
        0xE0B8 => key_release!(Meta(AltRight(false))),

        _ => None,
    }
}
