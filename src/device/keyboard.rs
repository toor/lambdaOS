use device::cpuio;
use device::vga::buffer::SCREEN;
use spin::Mutex;

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

struct Modifiers {
    shift: KeyPair,
    control: KeyPair,
    alt: KeyPair,
    caps_lock: bool,
}

impl Modifiers {
    const fn new() -> Self {
        Modifiers {
            shift: KeyPair::new(),
            control: KeyPair::new(),
            alt: KeyPair::new(),
            caps_lock: false,
        }
    }

    fn use_uppercase_letters(&self) -> bool {
        self.shift.is_pressed() ^ self.caps_lock
    }
    
    //Apply modifiers to ascii and return update ascii.
    fn apply_to(&self, ascii: u8) -> u8 {
        if b'a' <= ascii && ascii <= b'z' {
            if self.use_uppercase_letters() {
                return ascii - b'a' + b'A';
            }
        }

        ascii
    }
    
    //Update modifier state.
    fn update(&mut self, scancode: u8) {
        match scancode {
            0x1D => self.control.left = true,
            0x2A => self.shift.left = true,
            0x36 => self.shift.right = true,
            0x38 => self.alt.left = true,
            0x3A => self.caps_lock = !self.caps_lock,
            0x9D => self.control.left = false,
            0xAA => self.shift.left = false,
            0xB6 => self.shift.right = false,
            0xB8 => self.alt.right = false,

            _ => {},
        }
    }
}

struct State {
    port: cpuio::Port<u8>,
    modifiers: Modifiers,
}

static STATE: Mutex<State> = Mutex::new(State {
    port: unsafe { cpuio::Port::new(0x60) },
    modifiers: Modifiers::new(),
});

fn find_ascii(scancode: u8) -> Option<u8> {
    let idx = scancode as usize;

    let print = match idx {
        0x1e => 'a',
        0x30 => 'b',
        0x2e => 'c',
        0x20 => 'd',
        0x12 => 'e',
        0x21 => 'f',
        0x22 => 'g',
        0x23 => 'h',
        0x17 => 'i',
        0x24 => 'j',
        0x25 => 'k',
        0x26 => 'l',
        0x32 => 'm',
        0x31 => 'n',
        0x18 => 'o',
        0x19 => 'p',
        0x10 => 'q',
        0x13 => 'r',
        0x1f => 's',
        0x14 => 't',
        0x16 => 'u',
        0x2f => 'v',
        0x11 => 'w',
        0x2d => 'x',
        0x15 => 'y',
        0x2c => 'z',
        0x0b => '0',
        0x02 => '1',
        0x03 => '2',
        0x04 => '3',
        0x05 => '4',
        0x06 => '5',
        0x07 => '6',
        0x08 => '7',
        0x09 => '8',
        0x0a => '9',
        0x29 => '`',
        0x0c => '-',
        0x0d => '=',
        0x2b => '\\',
        0x39 => ' ',
        0x1a => '[',
        0x1b => ']',
        0x27 => ';',
        0x28 => '\'',
        0x33 => ',',
        0x34 => '.',
        0x35 => '/',
        0x37 => '*', // Keypad
        0x4a => '-', // Keypad
        0x4e => '+', // Keypad
        0x1c => {
            //Enter key.
            SCREEN.lock().new_line();
            return None;
        },

        0x0E => {
            //Backspace
            SCREEN.lock().delete_byte();
            return None;
        },
        _ => return None,
    };

    let printable = print as u8;

    Some(printable)
}

pub fn read_char() -> Option<char> {
    let mut state = STATE.lock();

    let scancode: u8 = state.port.read();

    state.modifiers.update(scancode);

    let scancode = scancode;

    if let Some(ascii) = find_ascii(scancode) {
        Some(state.modifiers.apply_to(ascii) as char)
    } else {
        //We have no clue how to handle this scancode. Pretend it didn't happen.
        None
    }
}
