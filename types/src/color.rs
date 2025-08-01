use std::ops::Index;

#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}
