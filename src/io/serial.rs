//Inspired by http://wiki.osdev.org/Serial_Ports
use core::fmt;
use spin::Mutex;
use io::cpuio;
use self::Register::*;

#[allow(dead_code)]
#[repr(C, u8)]

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
        ComPort { base_addr: base_addr, initialized: false}
    }
    
    //Return the COM port + register offset
    unsafe fn port(&mut self, register: Register) -> cpuio::Port<u8> {
        cpuio::Port::new(self.base_addr + (register as u8 as u16))
    }
    
    //Set the baud divisor. The UART internal clock runs at 115200 ticks, which we apply this
    //divisor to in order to get the BAUD rate.
    fn set_baud_divisor(&mut self, divisor: u16) {
        unsafe {
            
        }
    }
}
