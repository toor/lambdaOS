use spin::Mutex;
use device::vga::vga::{VGA, Color, ColorCode};
use core::fmt;

//Main print interface.
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    SCREEN.lock().write_fmt(args).unwrap();
}

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

pub struct TextBuffer {
    //Array of rows of characters.
    chars: [[u8; BUFFER_WIDTH]; BUFFER_HEIGHT],
    //How far along a row we are.
    column_position: usize,
    color_code: ColorCode,
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
        VGA.lock().update_cursor(BUFFER_HEIGHT -1, self.column_position);
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
            0x8 => self.delete_byte(),
            // Tab escape.
            b'\t' => {
                for _ in 0..4 {
                    self.write_byte(b' ');
                }
            }
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

        self.sync();
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
        self.sync();
    }
    
    /// Newline.
    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.chars[row - 1][col] = self.chars[row][col]
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        //Set position to start of row.
        self.column_position = 0;

        self.sync();
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

pub static SCREEN: Mutex<TextBuffer> = Mutex::new(TextBuffer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    chars: [[b' '; BUFFER_WIDTH]; BUFFER_HEIGHT],
});
