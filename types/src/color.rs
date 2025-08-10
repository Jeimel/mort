use std::ops::Index;

#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 2 {
            // Safety: `index` has a corresponding `Color` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    pub fn from_fen(color: &str) -> Self {
        if color == "w" {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}
