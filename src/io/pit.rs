use io::Port;
use spin::Mutex;

const SELECT_CHAN_0: u8 = 0;
const LOHI: u8 = 0x30;
const DIVISOR: u16 = 2685;

pub struct Pit {
    pub control: Port<u8>,
    pub chan0: Port<u8>,
}

impl Pit {
    pub const unsafe fn new(control: u16, chan0: u16) -> Self {
        Pit {
            control: Port::new(control),
            chan0: Port::new(chan0),
        }
    }
}

pub static PIT: Mutex<Pit> = Mutex::new(unsafe { Pit::new(0x43, 0x40) });

pub unsafe fn init() {
    PIT.lock().control.write(SELECT_CHAN_0 | LOHI | 5);
    PIT.lock().chan0.write((DIVISOR & 0xFF) as u8);
    PIT.lock().chan0.write((DIVISOR >> 8) as u8);

    println!("Using PIT controller");
}
