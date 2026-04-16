use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index},
    ptr, slice,
};

#[derive(Clone)]
pub struct ArrayVec<T: Copy, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T: Copy, const N: usize> ArrayVec<T, N> {
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn get(&self, index: usize) -> &T {
        debug_assert!(index < self.len);

        unsafe { &*self.data.get_unchecked(index).as_ptr() }
    }

    pub fn push(&mut self, value: T) {
        debug_assert!(self.len < N);

        unsafe {
            self.data[self.len].as_mut_ptr().write(value);
        };
        self.len += 1;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn extend(&mut self, other: &[T]) {
        debug_assert!(self.len() + other.len() < N);

        unsafe {
            let dst = self.as_mut_ptr().add(self.len());
            ptr::copy_nonoverlapping(other.as_ptr(), dst, other.len());
            self.len += other.len();
        }
    }
}

impl<T: Copy, const N: usize> Index<usize> for ArrayVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.data.get_unchecked(index).as_ptr() }
    }
}

impl<T: Copy, const N: usize> Deref for ArrayVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data.as_ptr().cast(), self.len) }
    }
}

impl<T: Copy, const N: usize> DerefMut for ArrayVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr().cast(), self.len) }
    }
}
