//! VGA - Interface to the VGA text buffer at physical address 0xb8000.

use device::vga::buffer::{TextBuffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// The possible colours that characters on the VGA buffer can be.
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
/// A representation of a VGA bg/fg colour code, calculated from byte-sized 
/// bg/fg data.
pub struct ColorCode(u8);

impl ColorCode {
    /// Create a new `ColorCode`.
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// A single character on the VGA text buffer.
pub struct ScreenChar {
    /// The 7-bit ascii character this `ScreenChar` represents.
    pub ascii_character: u8,
    /// The colour of this `ScreenChar`.
    pub color_code: ColorCode,
}

/// A 2D array of `ScreenChar`s, with 80*25 elements.
struct ScreenBuffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Interface to the VGA buffer.
pub struct Vga {
    /// Unique pointer to a screen buffer in physical memory.
    frame: Unique<ScreenBuffer>,
}

/// Static VGA interface. We cast the base address `0xb8000` of VGA memory to a `ScreenBuffer`
/// struct, which makes it useful to us.
pub static VGA: Mutex<Vga> = Mutex::new(Vga {
    frame: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

impl Vga {
    /// Return a reference to the `ScreenBuffer` pointer.
    fn frame(&mut self) -> &mut ScreenBuffer {
        unsafe { self.frame.as_mut() }
    }
    
    /// Sync the virtual `buffer` with the `ScreenBuffer` pointer.
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
    /// Update the text mode cursor to coordinates (x, y).
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
