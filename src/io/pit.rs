use io::Port;
use spin::Mutex;

/// Configuration data. Use channel 0 and mode 3, square wave generator. Use lohi operation.
const PIT_SET: u8 = 0x36;
static DIVISOR: u16 = 2685;

/// Simple interface to the PIT.
pub static PIT: Mutex<[Port<u8>; 2]> = Mutex::new(unsafe { [
    Port::new(0x43),
    Port::new(0x40),
]});

pub fn init() {
    PIT.lock()[0].write(PIT_SET);
    PIT.lock()[1].write((DIVISOR & 0xFF) as u8);
    PIT.lock()[1].write((DIVISOR >> 8) as u8);

    println!("[ OK ] Programmable Interval Timer.");
}
