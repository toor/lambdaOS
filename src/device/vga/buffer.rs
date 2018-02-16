use spin::Mutex;
use device::vga::vga::{Color, ColorCode, VGA};
use core::fmt;

/// Main print interface.
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    SCREEN.lock().write_fmt(args).unwrap();
}

/// The width of the VGA text buffer.
pub const BUFFER_WIDTH: usize = 80;
/// The height of the VGA text buffer.
pub const BUFFER_HEIGHT: usize = 25;

#[derive(Copy, Clone)]
/// A virtual text buffer.
pub struct TextBuffer {
    /// Array of rows of characters.
    pub chars: [[u8; BUFFER_WIDTH]; BUFFER_HEIGHT],
    /// How far along a row we are.
    pub column_position: usize,
    /// Represents the colour of the TTY buffer.
    pub color_code: ColorCode,
    pub active: bool,
}

/// Clear the VGA buffer.
pub fn clear_screen() {
    for _row in 0..BUFFER_HEIGHT {
        SCREEN.lock().new_line();
    }
}

impl TextBuffer {
    /// Sync this virtual text buffer with the actual VGA buffer at 0xb8000.
    fn sync(&self) {
        VGA.lock().sync_buffer(&self);
        VGA.lock()
            .update_cursor(BUFFER_HEIGHT - 1, self.column_position);
    }

    /// Return the current character array.
    pub fn chars(&self) -> &[[u8; BUFFER_WIDTH]; BUFFER_HEIGHT] {
        &self.chars
    }

    /// Return the current colour code.
    pub fn color_code(&self) -> ColorCode {
        self.color_code
    }

    /// Write a byte to the VGA buffer.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // Newline character.
            b'\n' => self.new_line(),
            // Backspace.
            0x8 => self.delete_byte(),
            // Tab escape.
            b'\t' => for _ in 0..4 {
                self.write_byte(b' ');
            },
            // Catch-all pattern that just updates the character array with the given byte.
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    // At end of row.
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                self.chars[row][col] = byte;
                self.column_position += 1;
            }
        }

        if self.active {
            self.sync();
        }
    }

    /// Delete a single byte from the buffer.
    pub fn delete_byte(&mut self) {
        if self.column_position == 0 {
            //At start of row, no bytes to delete.
            return;
        }

        let col = self.column_position - 1;

        self.chars[BUFFER_HEIGHT - 1][col] = b' ';
        self.column_position -= 1;
        
        if self.active {
            self.sync();
        }
    }

    /// Newline. This method will be called when a `\n` character is written
    /// to the virtual buffer.
    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.chars[row - 1][col] = self.chars[row][col]
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        //Set position to start of row.
        self.column_position = 0;

        if self.active {
            self.sync();
        }
    }

    /// Clear a single row by stepping across the entire width of the current row, and writing a
    /// blank character to each position.
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.chars[row][col] = b' ';
        }
    }
}

impl ::core::fmt::Write for TextBuffer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }

        Ok(())
    }
}

/// Global interface to the VGA text mode. 
pub static SCREEN: Mutex<TextBuffer> = Mutex::new(TextBuffer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGray, Color::Black),
    chars: [[b' '; BUFFER_WIDTH]; BUFFER_HEIGHT],
    active: true,
});

pub static TTYS: Mutex<Option<[TextBuffer; 6]>> = Mutex::new(None);

/// Switch `SCREEN` to `ttys[index]`.
pub fn switch(index: usize) {
    let inner = |idx: usize, list: &mut [TextBuffer; 6]| {
        list[idx].active = true;
        *SCREEN.lock() = list[idx]; 
    };

    let mut list = *TTYS.lock();

    let list = match list {
        Some(ref mut t) => t,
        None => panic!("TTY list called before init."),
    };

    // Only gets called if `list` is Some
    inner(index, list);
}

/// Initialise all the TTYS.
pub fn tty_init() {
    // Create six identical TTYS.
    let buffers: [TextBuffer; 6] = [TextBuffer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        chars: [[b' '; BUFFER_WIDTH]; BUFFER_HEIGHT],
        active: false,
    }; 6];

    *TTYS.lock() = Some(buffers);
}
