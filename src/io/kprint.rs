use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::mem;
use alloc::string::String;
use io::serial;
use spin::Mutex;

pub struct KPrintBuffer {
    buffer: Box<Vec<String>>,
    partial: String,
}

impl core::fmt::Write for KPrintBuffer {
    fn write_str(&mut self, ss: &str) -> core::fmt::Result {
        
        let mut s = String::from(ss);
        let endline = ss.find('\n').unwrap_or(255);

        match endline {
            255 => self.partial += ss,
            _ => {
                let remainder = s.split_off(endline);
                let line = self.partial.clone() + &s;
                serial::write(&line);
                self.buffer.push(line);
                self.partial = remainder;
            }
        };

        Ok(())
    }
}

static mut KPRINT_BUFFER: Option<Mutex<KPrintBuffer>> = None;

pub fn init() {
    unsafe {
        KPRINT_BUFFER = Some(Mutex::new(KPrintBuffer {
            buffer: Box::new(vec!()),
            partial: String::new(),
        }));
    }
}

pub fn print(args: fmt::Arguments) {
    unsafe {
        match KPRINT_BUFFER {
            Some(ref mut kp) => {
                use core::fmt::Write;
                let mut pb = kp.lock();
                (*pb).write_fmt(args).unwrap()
            },
            None => {},
        }
    }
}