//Inspired by http://wiki.osdev.org/Serial_Ports
use core::fmt;
use spin::Mutex;
use io::cpuio;
use self::Register::*;

#[allow(dead_code)]
#[repr(u8)]
enum Register {
    DataOrBaudLsb = 0,
    InterruptEnableOrBaudMsb = 1,
    InterruptIdAndFifo = 2,
    LineControlReg = 3,
    ModemControlReg = 4,
    LineStatusReg = 5,
    ModemStatusReg = 6,
    ScratchReg = 7,
}

pub struct ComPort {
    base_addr: u16, //The base address of the COM port, identified as the base of the associated I/O registers.
    initialized: bool, //Has this port been initialized yet?
}

impl ComPort {
    const unsafe fn new(base_addr: u16) -> ComPort {
        ComPort {
            base_addr: base_addr,
            initialized: false,
        }
    }

    unsafe fn lazy_initialize(&mut self) {
        if self.initialized == true {
            return;
        }
        self.initialized = true;

        //Disable interrupts.
        self.port(InterruptEnableOrBaudMsb).write(0x00);

        //Set Baud and 8N1 mode.
        self.set_baud_divisor(2); //115,200 divided by 2.
        self.port(LineControlReg).write(0x03);

        //Enable interrupt FIFOs with 14-byte threshold.
        self.port(InterruptIdAndFifo).write(0xC7);

        //Configure the modem as having RTS/DSR and IRQs on.
        self.port(ModemControlReg).write(0x0B);
    }

    //Return the COM port + register offset
    unsafe fn port(&mut self, register: Register) -> cpuio::Port<u8> {
        cpuio::Port::new(self.base_addr + (register as u8 as u16))
    }

    //Set the baud divisor. The UART internal clock runs at 115200 ticks, which we apply this
    //divisor to in order to get the BAUD rate.
    fn set_baud_divisor(&mut self, divisor: u16) {
        unsafe {
            self.lazy_initialize();
            let saved_line_control = self.port(LineControlReg).read();
            self.port(LineControlReg).write(0x80 | saved_line_control);


            self.port(DataOrBaudLsb).write(divisor as u8);
            self.port(InterruptEnableOrBaudMsb)
                .write((divisor >> 8) as u8);

            //Restore old port modes.
            self.port(LineControlReg).write(saved_line_control);
        }
    }

    fn can_transmit(&mut self) -> bool {
        unsafe {
            self.lazy_initialize();

            (self.port(LineStatusReg).read() & 0x20) != 0
        }
    }
}

impl fmt::Write for ComPort {
    //Output a string to the com port.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            self.lazy_initialize();

            //Output each byte (character) one-by-one.
            for b in s.bytes() {
                //Loop until we can get a hold of the port.
                while !self.can_transmit() {}

                //Write the byte.
                self.port(DataOrBaudLsb).write(b);
            }
        }

        Ok(())
    }
}


//Our primary serial port.
pub static COM1: Mutex<ComPort> = Mutex::new(unsafe { ComPort::new(0x03F8) });
