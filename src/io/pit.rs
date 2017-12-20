use io::Port;
use spin::Mutex;

const SELECT_CHAN_0: u8 = 0;
const LOHI: u8 = 0x30;
const DIVISOR: u16 = 2685;

const PIT_A: u16 = 0x43;
const PIT_CONTROL: u16 = 0x40;
const PIT_MASK: u8 = 0xFF;
const PIT_SCALE: u32 = 1193180;
/// Select Mode 0, lobyte/hibyte operation, mode 3: square wave generator.
/// Set BCD as 0 - 16-bit binary.
const PIT_SET: u8 = 0x36;
const SUBTICKS: u16 = 1000;

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

pub fn init() {
    let mut divisor: u32 = PIT_SCALE / SUBTICKS as u32;
    
    let config_1: u8 = (divisor & (PIT_MASK as u32)) as u8;
    let config_2: u8 = ((divisor >> 8) & (PIT_MASK as u32)) as u8;
    
    // Setup using mode data.
    PIT.lock().control.write(PIT_SET);
    
    // We are now free to configure the divisor via the Channel 0 port.
    PIT.lock().chan0.write(config_1);
    PIT.lock().chan0.write(config_2);

    println!("Using PIT controller");
}
