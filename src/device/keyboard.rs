use device::io::cpuio;
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
