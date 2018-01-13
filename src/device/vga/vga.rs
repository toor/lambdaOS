//VGA - Interface to the magical VGA text buffer at physical address 0xb8000.

use device::vga::buffer::{TextBuffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

struct ScreenBuffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Vga {
    frame: Unique<ScreenBuffer>,
}

pub static VGA: Mutex<Vga> = Mutex::new(Vga {
    frame: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

impl Vga {
    fn frame(&mut self) -> &mut ScreenBuffer {
        unsafe { self.frame.as_mut() }
    }

    pub fn sync_buffer(&mut self, buffer: &TextBuffer) {
        let frame = self.frame();

        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                //Update using the text buffer.
                let character = ScreenChar {
                    ascii_character: buffer.chars()[row][col],
                    color_code: buffer.color_code(),
                };

                frame.chars[row][col].write(character);
            }
        }
    }

    #[allow(exceeding_bitshifts)]
    pub fn update_cursor(&self, row: usize, col: usize) {
        let pos = ((BUFFER_WIDTH as u16) * (row as u16)) + col as u16;
        use device::Port;

        unsafe {
            let mut control_port: Port<u8> = Port::new(0x3D4);
            let mut value_port: Port<u8> = Port::new(0x3D5);

            control_port.write(0x0F);
            value_port.write((pos & 0xFF) as u8);
            control_port.write(0x0E);
            value_port.write(((pos >> 8) & 0xFF) as u8);
        }
    }
}
