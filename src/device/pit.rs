use device::Port;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

/// Configuration data. Use channel 0 and mode 3, square wave generator. Use lohi operation.
const PIT_SET: u8 = 0x36;
static DIVISOR: u16 = 2685;

/// Simple interface to the PIT.
pub static PIT: Mutex<[Port<u8>; 2]> = Mutex::new(unsafe { [Port::new(0x43), Port::new(0x40)] });

pub fn init() {
    println!("[ dev ] Setting pit mode.");
    PIT.lock()[0].write(PIT_SET);
    println!("[ dev ] Setting up frequency.");
    PIT.lock()[1].write((DIVISOR & 0xFF) as u8);
    PIT.lock()[1].write((DIVISOR >> 8) as u8);
    
    let mut frequency: u32 = 1193182 / 2685;

    let irq0_int_timeout = {
        let val = 1 / frequency;
        val * 1000
    };

    println!("[ dev ] Initialising PIT, setup to interrupt every {} ms", irq0_int_timeout);
}

pub static PIT_TICKS: AtomicUsize = ATOMIC_USIZE_INIT;
