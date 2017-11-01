use core::fmt::{self, Write};
use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

pub static SCREEN: Mutex<Screen> = Mutex::new(Screen {
    color_code: ColorCode::new(Color::LightGreen, Color::DarkGrey),
    col_pos: 0,
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    SCREEN.lock().write_fmt(args).unwrap();
}

//Standard VGA colors.
#[derive(Copy, Clone)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

//VGA foreground and background color set.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//A colored VGA character.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Char {
    pub code: u8,
    pub colors: ColorCode,
}

struct Buffer {
    chars: [[Volatile<Char>; WIDTH]; HEIGHT],
}

//A VGA screen, in character mode.
pub struct Screen {
    color_code: ColorCode,
    col_pos: usize,
    buffer: Unique<Buffer>,
}

impl Screen {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                //Row filled.
                if self.column_position >= WIDTH {
                    self.new_line();
                }
                let row = HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;

                self.buffer().chars[row][col].write(Char {
                    code: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() } 
    }

    fn new_line(&mut self) {
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            code: b' ',
            color_code: self.color_code,
        };
    }
}

impl Write for Screen {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}
