pub mod buffer;
pub mod vga;

pub fn init() {
    self::buffer::tty_init();
}
