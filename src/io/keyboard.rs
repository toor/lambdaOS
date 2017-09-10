use spin::Mutex;
use cpuio;


//A pair of keys which appear on both sides of the keyboard, i.e the alt keys.
#[derive(Debug)]
struct KeyPair {
    left: bool,
    right: bool,
}

impl KeyPair {
    const fn new() -> Self {
        KeyPair { left: false, right: false}
    }

    //Check if either of these keys is pressed.
    fn is_pressed(&self) -> bool {
        self.left || self.right
    }
}

struct Modifiers {
    shift: KeyPair,
    ctrl: KeyPair,
    alt: KeyPair,
    caps_lock: bool,
}

impl Modifiers {
    const fn new() -> Self {
        Modifiers {
            shift: KeyPair::new(),
            ctrl: KeyPair::new(),
            alt: KeyPair::new(),
            caps_lock: false,
        }
    }

    //Should letters outputted be uppercase based on shift/caps?
    fn use_uppercase(&self) -> bool {
        self.shift.is_pressed() ^ self.caps_lock
    }

    //Apply these modifiers and return the new character
    fn apply_to(&self, ascii: u8) -> u8 {
        if b'a' <= ascii && ascii <= b'z' {
            if self.use_uppercase() {
                return ascii - b'a' + b'A';
            }
        }

        ascii
    }

    fn update(&mut self, scancode: u8) {
        match scancode {
            0x1D => self.ctrl.left = true,
            0x2A => self.shift.left = true,
            0x36 => self.shift.right = true,
            0x38 => self.alt.left = true,
            //Caps toggles on the leading edge.
            0x3A => self.caps_lock = !self.caps_lock,
            0x9D => self.control.left = false,
            0xAA => self.shift.left = false,
            0xB6 => self.shift.right = false,
            0xB8 => self.alt.left = false,

            _ => {},
        }
    }
}

//Our keyboard state, including port information and info about the modifiers currently in use.
struct State {
    //Rather than reading from the 0x60 keyboard port directly, let's wrap it in a byte-sized port
    //object.
    port: cpuio::Port<u8>,

    modifiers: Modifiers,
}

//Our global keyboard state, protected by a Mutex.
static STATE: Mutex<State> = Mutex::new(State {
    port: unsafe { cpuio::Port::new(0x60) },
    modifiers: Modifiers::new(),
});

fn find_ascii(scancode: u8) -> Option<u8> {
    let idx = scancode as usize;
    match scancode {
        0x01 ... 0x0E => Some(b"\x1B1234567890-=\0x02"[idx-0x01]),
        0x0F ... 0x1C => Some(b"\tqwertyuiop[]\r"[idx-0x0F]),
        0x1E ... 0x28 => Some(b"asdfghjkl;'"[idx-0x1E]),
        0x2C ... 0x35 => Some(b"zxcvbnm,./"[idx-0x2C]),
        0x39 => Some(b' '),
        _ => None,
    }
}

//Try reading a single input character.
pub fn read_char() -> Option<char> {
    let mut state = STATE.lock();

    //Read one scancode.
    let scancode = state.port.read();

    //Modifiers get first to update based on the scancode.
    state.modifiers.update(scancode);

    if let Some(ascii) = find_ascii(scancode) {
        Some(state.modifiers.apply_to(ascii) as char)
    } else {
        //Either this was a modifier scancode, or we don't know what this scancode does. Just
        //pretend nothing happened
        None
    }
}
