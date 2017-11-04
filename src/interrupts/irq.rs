use super::PICS;
use io::cpuio::Port;
use io::keyboard;

pub extern "x86-interrupt" fn timer_handler() {
    PICS.lock().notify_end_of_interrupt(0x20);
}

pub extern "x86-interrupt" fn keyboard_handler() {
    PICS.lock().notify_end_of_interrupt(0x21);

    let mut port = unsafe { Port::new(0x60); }

    let scancode: u8 = port.read();

    if let Some(c) = keyboard::ascii_from_scancode() {
        println!("{}", c);
    }
}
