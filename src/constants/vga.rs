pub use io::drivers::display::vga::{ScreenChar, Color, ColorCode};

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub const BUFFER_START: usize = 0xb8000; //The starting address of the vga buffer

pub const GREEN_BLANK: ScreenChar = ScreenChar {
    ascii_character: b'-',
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
};

pub const RED_BLANK: ScreenChar = ScreenChar {
    ascii_character: b' ',
    color_code: ColorCode::new(Color::LightRed, Color::Black),
};

pub const GRAY_BLANK: ScreenChar = ScreenChar {
    ascii_character: b' ',
    color_code: ColorCode::new(Color::LightGray, Color::Black),
};