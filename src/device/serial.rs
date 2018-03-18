use device::io::cpuio::Port;
use self::Register::*;
use spin::Mutex;

/// An interface to a serial port.
pub struct SerialPort {
    base: u16,
    is_initialized: bool,
}

impl SerialPort {
    const unsafe fn new(base: u16) -> SerialPort {
        SerialPort {
            base: base,
            is_initialized: false,
        }
    }
    
    pub fn do_init(&mut self) {
        // Check if this function has already been called.
        if self.is_initialized == true { return; }
        self.is_initialized = true;

        // Disable interrupts.
        self.port(IntEnableOrMsb).write(0x00);
        // Enable DLAB.
        self.port(LineControl).write(0x80);
        // Set divisor as 2.
        self.port(DataOrBaudLsb).write(0x02);
        self.port(IntEnableOrMsb).write(0x00);
        // 8 bits, no parity, one stop bit.
        self.port(LineControl).write(0x03);
        self.port(InterruptIdentAndFifo).write(0xc7);
        self.port(ModemControl).write(0x0b);
        // Done!
    }
    
    /// Check if it is safe to read from this port.
    fn can_read(&mut self) -> bool {
        (self.port(LineStatus).read() & 1) == 0        
    }
    
    /// Wait until we can get a hold on the data register, and then read from the serial port.
    pub fn read_serial(&mut self) -> u8 {
        while self.can_read() {}
        
        self.port(DataOrBaudLsb).read()
    }
    
    /// Check if we can safely write the data to the serial port.
    fn is_transmit_empty(&mut self) -> bool {
        (self.port(LineStatus).read() & 0x20) == 0
    }
    
    /// Wait until we can get a hold on the data register, and then write to the serial port.
    pub fn write_serial(&mut self, data: u8) {
        while self.is_transmit_empty() {}

        self.port(DataOrBaudLsb).write(data);
    }

    fn port(&mut self, register: Register) -> Port<u8> {
        unsafe { Port::new(self.base + (register as u8 as u16)) }
    }
}

#[repr(C, u8)]
/// Serial port registers.
enum Register {
    DataOrBaudLsb = 0,
    IntEnableOrMsb = 1,
    InterruptIdentAndFifo = 2,
    LineControl = 3,
    ModemControl = 4,
    LineStatus = 5,
    ModemStatus = 6,
    Scratch = 7,
}

pub static COM1: Mutex<SerialPort> = Mutex::new(unsafe {
    SerialPort::new(0x3f8)
});
