use super::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_code: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii_code: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_code,
            color_code,
        }
    }
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
