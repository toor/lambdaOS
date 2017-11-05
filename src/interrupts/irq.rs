use x86_64::structures::idt::ExceptionStackFrame;

use super::PICS;
use io::cpuio::Port;
use io::keyboard;

pub extern "x86-interrupt" fn timer_handler(stack_frame: &mut ExceptionStackFrame) {
    unsafe { PICS.lock().notify_end_of_interrupt(0x20) };
}

pub extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame) {
    let mut port = unsafe { Port::new(0x60) };
    
    let scancode: u8 = port.read();

    if let Some(c) = keyboard::scancode_to_ascii(scancode as usize) {
        println!("{}", c);
    }

    unsafe {  PICS.lock().notify_end_of_interrupt(0x21) };
}
