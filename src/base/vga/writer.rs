use core::fmt::{self, Write};

use super::{
    buffer::{Buffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH},
    color::ColorCode,
};

use lazy_static::lazy_static;
use spin::Mutex;

pub struct Writer {
    // (row, col)
    cursor_position: (usize, usize),
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        cursor_position: (0, 0),
        color_code: ColorCode::new(super::color::Color::White, super::color::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cursor_position.1 >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.cursor_position.0;
                let col = self.cursor_position.1;

                unsafe {
                    self.write_char((row, col), ScreenChar::new(byte, self.color_code));
                }
                self.cursor_position.1 += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);

        for i in 0..BUFFER_WIDTH {
            unsafe { self.write_char((row, i), blank) };
        }
    }

    fn new_line(&mut self) {
        if self.cursor_position.0 >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let char = self.buffer.chars[row][col];
                    unsafe { self.write_char((row - 1, col), char) };
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.cursor_position.0 += 1;
        }
        self.cursor_position.1 = 0;
    }

    unsafe fn write_char(&mut self, pos: (usize, usize), char: ScreenChar) {
        core::ptr::write_volatile(
            &mut self.buffer.chars[pos.0][pos.1] as *mut ScreenChar,
            char,
        );
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
