use core::fmt;
use spin::Mutex;
use vga;

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        try!(vga::SCREEN.lock().write_str(s))
    }
}

pub static CONSOLE: Mutex<Console> = Mutex::new(Console);
