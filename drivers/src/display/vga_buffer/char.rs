use super::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_character,
            color_code,
        }
    }
}
