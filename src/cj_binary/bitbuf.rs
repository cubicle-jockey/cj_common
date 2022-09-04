//! structs, methods and traits for getting/setting bits at given positions of the implemented types
//!
//! # Quick Start
//!
//!```
//! use cj_common::prelude::*;
//!
//! let x = 0b00000010u8;
//! assert_eq!(x.get_bit(1),true);
//!
//! let mut x = 0b00000000u8;
//! x.set_bit(1,true);
//! assert_eq!(x,0b00000010u8);
//!
//! // iter example
//! let x = 0xABu8;
//! let mut v = Vec::new();
//! for i in x.bit_iter() {
//!     v.push(i);
//! }
//! assert_eq!(
//!    v.as_slice(),
//!    &[true, true, false, true, false, true, false, true]
//! );
//! ```

/// iterator for the BitFlag trait
pub struct BitIter<'a, T> {
    byte_count: usize,
    index: usize,
    inner: &'a T,
}

impl<T: Bitflag + Sized> BitIter<'_, T> {
    pub fn new(byte_count: usize, value: &T) -> BitIter<T>
    where
        T: Bitflag + Sized,
    {
        BitIter {
            byte_count,
            index: 0,
            inner: value,
        }
    }

    fn next_bit(&mut self) -> Option<bool>
    where
        T: Bitflag + Sized,
    {
        if self.index < self.byte_count {
            let r = self.inner.get_bit(self.index);
            self.index += 1;
            Some(r)
        } else {
            None
        }
    }
}

impl<T: Bitflag + Sized> Iterator for BitIter<'_, T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_bit()
    }
}

/// trait to implement bit_iter() for instantiating BitIter
///
/// - implemented for u8, u16, u32, u64 and u128
/// ```
/// # use cj_common::prelude::BitFlagIter;
/// let x = 0xABu8;
///
/// let mut i = x.bit_iter();
/// assert_eq!(i.next(), Some(true));
/// assert_eq!(i.next(), Some(true));
/// assert_eq!(i.next(), Some(false));
/// assert_eq!(i.next(), Some(true));
/// ```
///
/// ```
/// # use cj_common::prelude::BitFlagIter;
/// let x = 0xABu8;
/// let mut v = Vec::new();
/// for i in x.bit_iter() {
///    v.push(i);
/// }
///
/// assert_eq!(
///     v.as_slice(),
///     &[true, true, false, true, false, true, false, true]
/// );
/// ```
pub trait BitFlagIter<'a, T> {
    fn bit_iter(&'a self) -> BitIter<'a, T>;
}

impl<'a> BitFlagIter<'a, u8> for u8 {
    fn bit_iter(&'a self) -> BitIter<'a, u8> {
        BitIter::new(8, self)
    }
}

impl<'a> BitFlagIter<'a, u16> for u16 {
    fn bit_iter(&'a self) -> BitIter<'a, u16> {
        BitIter::new(16, self)
    }
}

impl<'a> BitFlagIter<'a, u32> for u32 {
    fn bit_iter(&'a self) -> BitIter<'a, u32> {
        BitIter::new(32, self)
    }
}

impl<'a> BitFlagIter<'a, u64> for u64 {
    fn bit_iter(&'a self) -> BitIter<'a, u64> {
        BitIter::new(64, self)
    }
}

impl<'a> BitFlagIter<'a, u128> for u128 {
    fn bit_iter(&'a self) -> BitIter<'a, u128> {
        BitIter::new(128, self)
    }
}

/// trait for implementing get_bit and set_bit methods.  These methods represent bit as bool and are used to get/set bits at given positions of the implemented types.
///
/// - implemented for u8, u16, u32, u64 and u128
pub trait Bitflag {
    /// returns true if the bit value at the specified position is set.
    /// * false will be returned if the bit is not set, or if the bit_pos is out of range
    /// ```
    /// # use cj_common::prelude::Bitflag;
    /// let x = 0b00000010u8;
    /// assert_eq!(x.get_bit(1),true);
    /// ```
    fn get_bit(&self, bit_pos: usize) -> bool;
    /// sets the bit value at the specified position.
    /// * the call is ignored if the bit_pos is out of range
    /// ```
    /// # use cj_common::prelude::Bitflag;
    /// let mut x = 0b00000000u8;
    /// x.set_bit(1,true);
    /// assert_eq!(x,0b00000010u8);
    /// ```
    fn set_bit(&mut self, bit_pos: usize, value: bool);
}

impl Bitflag for u8 {
    fn get_bit(&self, bit_pos: usize) -> bool {
        match bit_pos {
            0..=7 => {
                let v = 1u8 << bit_pos;
                self & v == v
            }
            _ => false,
        }
    }

    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        match bit_pos {
            0..=7 => {
                let v = 1u8 << bit_pos;
                let i = 0xFF - v;
                if value {
                    *self = (*self & i) + v;
                } else {
                    *self &= i;
                }
            }
            _ => {}
        }
    }
}

impl Bitflag for u16 {
    fn get_bit(&self, bit_pos: usize) -> bool {
        match bit_pos {
            0..=15 => {
                let v = 1u16 << bit_pos;
                self & v == v
            }
            _ => false,
        }
    }

    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        match bit_pos {
            0..=15 => {
                let v = 1u16 << bit_pos;
                let i = 0xFFFF - v;
                if value {
                    *self = (*self & i) + v;
                } else {
                    *self &= i;
                }
            }
            _ => {}
        }
    }
}

impl Bitflag for u32 {
    fn get_bit(&self, bit_pos: usize) -> bool {
        match bit_pos {
            0..=31 => {
                let v = 1u32 << bit_pos;
                self & v == v
            }
            _ => false,
        }
    }

    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        match bit_pos {
            0..=31 => {
                let v = 1u32 << bit_pos;
                let i = 0xFFFFFFFF - v;
                if value {
                    *self = (*self & i) + v;
                } else {
                    *self &= i;
                }
            }
            _ => {}
        }
    }
}

impl Bitflag for u64 {
    fn get_bit(&self, bit_pos: usize) -> bool {
        match bit_pos {
            0..=63 => {
                let v = 1u64 << bit_pos;
                self & v == v
            }
            _ => false,
        }
    }

    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        match bit_pos {
            0..=63 => {
                let v = 1u64 << bit_pos;
                let i = 0xFFFFFFFFFFFFFFFF - v;
                if value {
                    *self = (*self & i) + v;
                } else {
                    *self &= i;
                }
            }
            _ => {}
        }
    }
}

impl Bitflag for u128 {
    fn get_bit(&self, bit_pos: usize) -> bool {
        match bit_pos {
            0..=127 => {
                let v = 1u128 << bit_pos;
                self & v == v
            }
            _ => false,
        }
    }

    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        match bit_pos {
            0..=127 => {
                let v = 1u128 << bit_pos;
                let i = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF - v;
                if value {
                    *self = (*self & i) + v;
                } else {
                    *self &= i;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cj_binary::bitbuf::{BitFlagIter, Bitflag};

    #[test]
    fn test_u8_get() {
        let x = 0xABu8;
        assert_eq!(x.get_bit(0), true);
        assert_eq!(x.get_bit(1), true);
        assert_eq!(x.get_bit(2), false);
        assert_eq!(x.get_bit(100), false);
    }

    #[test]
    fn test_u8_set() {
        let mut x = 0x00u8;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(0), true);

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert_eq!(x.get_bit(1), true);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);
    }

    #[test]
    fn test_u16_get() {
        let x = 0xABu16;
        assert_eq!(x.get_bit(0), true);
        assert_eq!(x.get_bit(1), true);
        assert_eq!(x.get_bit(2), false);
        assert_eq!(x.get_bit(100), false);
    }

    #[test]
    fn test_u16_set() {
        let mut x = 0x00u16;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(0), true);

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert_eq!(x.get_bit(1), true);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);
    }

    #[test]
    fn test_u32_get() {
        let x = 0xABu32;
        assert_eq!(x.get_bit(0), true);
        assert_eq!(x.get_bit(1), true);
        assert_eq!(x.get_bit(2), false);
        assert_eq!(x.get_bit(100), false);
    }

    #[test]
    fn test_u32_set() {
        let mut x = 0x00u32;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(0), true);

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert_eq!(x.get_bit(1), true);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);
    }

    #[test]
    fn test_u64_get() {
        let x = 0xABu64;
        assert_eq!(x.get_bit(0), true);
        assert_eq!(x.get_bit(1), true);
        assert_eq!(x.get_bit(2), false);
        assert_eq!(x.get_bit(100), false);
    }

    #[test]
    fn test_u64_set() {
        let mut x = 0x00u64;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(0), true);

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert_eq!(x.get_bit(1), true);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);
    }

    #[test]
    fn test_u128_get() {
        let x = 0xABu128;
        assert_eq!(x.get_bit(0), true);
        assert_eq!(x.get_bit(1), true);
        assert_eq!(x.get_bit(2), false);
        assert_eq!(x.get_bit(100), false);
    }

    #[test]
    fn test_u128_set() {
        let mut x = 0x00u128;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(0), true);

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert_eq!(x.get_bit(1), true);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert_eq!(x.get_bit(1), false);
    }

    #[test]
    fn test_u8_iter() {
        let x = 0xABu8;

        let mut i = x.bit_iter();
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(false));
        assert_eq!(i.next(), Some(true));
    }

    #[test]
    fn test_u16_iter() {
        let x = 0xABu16;

        let mut i = x.bit_iter();
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(false));
        assert_eq!(i.next(), Some(true));
    }

    #[test]
    fn test_u32_iter() {
        let x = 0xABu32;

        let mut i = x.bit_iter();
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(false));
        assert_eq!(i.next(), Some(true));
    }

    #[test]
    fn test_u64_iter() {
        let x = 0xABu64;

        let mut i = x.bit_iter();
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(false));
        assert_eq!(i.next(), Some(true));
    }

    #[test]
    fn test_u128_iter() {
        let x = 0xABu128;

        let mut i = x.bit_iter();
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(true));
        assert_eq!(i.next(), Some(false));
        assert_eq!(i.next(), Some(true));
    }

    #[test]
    fn test_u8_iter2() {
        let x = 0xABu8;
        let mut v = Vec::new();
        for i in x.bit_iter() {
            v.push(i);
        }

        assert_eq!(
            v.as_slice(),
            &[true, true, false, true, false, true, false, true]
        );
    }
}
