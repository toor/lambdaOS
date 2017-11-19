//VGA - Interface to the magical VGA text buffer at physical address 0xb8000.


use io::vga::buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, TextBuffer};
use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

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
