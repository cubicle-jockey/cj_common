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
//!
//! // iter over vec example
//! let x = vec![0xABu8, 0xAB, 0xAB];
//! let mut v = Vec::new();
//! for i in x.iter_to_bit() {
//!     v.push(i);
//! }
//!
//! assert_eq!(
//!     v.as_slice(),
//!     &[
//!         true, true, false, true, false, true, false, true,
//!         true, true, false, true, false, true, false, true,
//!         true, true, false, true, false, true, false, true,
//!     ]
//! );
//!
//! // iter over slice example
//! let x = [2u128, 2, 2];
//! for i in x.as_slice().iter_to_bit().enumerate() {
//!     match i.0 {
//!         1 | 129 | 257 => assert_eq!(i.1, true),
//!         _ => assert_eq!(i.1, false),
//!     }
//! }
//!
//! // mask examples
//! let mask = 0b00011010u8;
//! let byte = 0b01011010u8;
//! assert_eq!(byte.matches_mask(&mask),true);
//! assert_eq!(mask.as_mask_matches(&byte),true);
//!
//! let read_permission  = 0b00000001u8;
//! let write_permission = 0b00000010u8;
//! let mod_permission   = 0b00000100u8;
//! let del_permission   = 0b00001000u8;
//! let full_permission  = read_permission + write_permission + mod_permission + del_permission;
//! let user             = read_permission + write_permission;
//! let moderator        = user + mod_permission;
//! let admin            = full_permission;
//!
//! let fred = user;
//! let jane = moderator;
//! assert_eq!(fred.matches_mask(&read_permission),true);
//! assert_eq!(fred.matches_mask(&write_permission),true);
//! assert_eq!(fred.matches_mask(&moderator),false);
//! assert_eq!(user.as_mask_matches(&jane),true);
//! assert_eq!(admin.as_mask_matches(&jane),false);
//! ```

use std::mem::size_of;
use std::slice::Iter;

/// iterator for the BitFlag trait
pub struct BitIter<'a, T> {
    byte_count: usize,
    index: usize,
    inner: &'a T,
}

impl<T: Bitflag + Sized> BitIter<'_, T> {
    pub fn new(byte_count: usize, value: &T) -> BitIter<'_, T>
    where
        T: Bitflag + Sized,
    {
        BitIter {
            byte_count,
            index: 0,
            inner: value,
        }
    }
    #[inline]
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
    #[inline]
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
    /// Iter for iterating over each bit of binary data returning true for 1 and false for 0
    ///
    /// implemented for u8, u16, u32, u64 and u128
    fn bit_iter(&'a self) -> BitIter<'a, T>;
}

impl<'a> BitFlagIter<'a, u8> for u8 {
    #[inline]
    fn bit_iter(&'a self) -> BitIter<'a, u8> {
        BitIter::new(8, self)
    }
}

impl<'a> BitFlagIter<'a, u16> for u16 {
    #[inline]
    fn bit_iter(&'a self) -> BitIter<'a, u16> {
        BitIter::new(16, self)
    }
}

impl<'a> BitFlagIter<'a, u32> for u32 {
    #[inline]
    fn bit_iter(&'a self) -> BitIter<'a, u32> {
        BitIter::new(32, self)
    }
}

impl<'a> BitFlagIter<'a, u64> for u64 {
    #[inline]
    fn bit_iter(&'a self) -> BitIter<'a, u64> {
        BitIter::new(64, self)
    }
}

impl<'a> BitFlagIter<'a, u128> for u128 {
    #[inline]
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
    #[inline]
    fn get_bit(&self, bit_pos: usize) -> bool {
        if let 0..=7 = bit_pos {
            let v = 1u8 << bit_pos;
            self & v == v
        } else {
            false
        }
    }

    #[inline]
    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        if let 0..=7 = bit_pos {
            let v = 1u8 << bit_pos;
            let i = 0xFF - v;
            if value {
                *self = (*self & i) + v;
            } else {
                *self &= i;
            }
        }
    }
}

impl Bitflag for u16 {
    #[inline]
    fn get_bit(&self, bit_pos: usize) -> bool {
        if let 0..=15 = bit_pos {
            let v = 1u16 << bit_pos;
            self & v == v
        } else {
            false
        }
    }
    #[inline]
    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        if let 0..=15 = bit_pos {
            let v = 1u16 << bit_pos;
            let i = 0xFFFF - v;
            if value {
                *self = (*self & i) + v;
            } else {
                *self &= i;
            }
        }
    }
}

impl Bitflag for u32 {
    #[inline]
    fn get_bit(&self, bit_pos: usize) -> bool {
        if let 0..=31 = bit_pos {
            let v = 1u32 << bit_pos;
            self & v == v
        } else {
            false
        }
    }
    #[inline]
    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        if let 0..=31 = bit_pos {
            let v = 1u32 << bit_pos;
            let i = 0xFFFFFFFF - v;
            if value {
                *self = (*self & i) + v;
            } else {
                *self &= i;
            }
        }
    }
}

impl Bitflag for u64 {
    #[inline]
    fn get_bit(&self, bit_pos: usize) -> bool {
        if let 0..=63 = bit_pos {
            let v = 1u64 << bit_pos;
            self & v == v
        } else {
            false
        }
    }
    #[inline]
    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        if let 0..=63 = bit_pos {
            let v = 1u64 << bit_pos;
            let i = 0xFFFFFFFFFFFFFFFF - v;
            if value {
                *self = (*self & i) + v;
            } else {
                *self &= i;
            }
        }
    }
}

impl Bitflag for u128 {
    #[inline]
    fn get_bit(&self, bit_pos: usize) -> bool {
        if let 0..=127 = bit_pos {
            let v = 1u128 << bit_pos;
            self & v == v
        } else {
            false
        }
    }
    #[inline]
    fn set_bit(&mut self, bit_pos: usize, value: bool) {
        if let 0..=127 = bit_pos {
            let v = 1u128 << bit_pos;
            let i = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF - v;
            if value {
                *self = (*self & i) + v;
            } else {
                *self &= i;
            }
        }
    }
}

/// bit iterator for iterating over Vec or Slice of u8, u16, u32, u64 and u128.
///
/// calling next will iterate over bits of each item until all items are exhausted.
/// ```
/// # use cj_common::prelude::*;
/// let x = vec![0xABu8, 0xAB, 0xAB];
/// let mut v = Vec::new();
/// for i in x.iter_to_bit() {
///     v.push(i);
/// }
///
/// assert_eq!(
///     v.as_slice(),
///     &[
///         true, true, false, true, false, true, false, true,
///         true, true, false, true, false, true, false, true,
///         true, true, false, true, false, true, false, true
///     ]
/// );
///
/// let x = [2u128, 2, 2];
/// for i in x.as_slice().iter_to_bit().enumerate() {
///     match i.0 {
///         1 | 129 | 257 => assert_eq!(i.1, true),
///         _ => assert_eq!(i.1, false),
///     }
/// }
/// ```

// TO.DO these need moved out of bitbuf and into a dedicated file
const DEF_U8: &u8 = &0;
const DEF_U16: &u16 = &0;
const DEF_U32: &u32 = &0;
const DEF_U64: &u64 = &0;
const DEF_U128: &u128 = &0;

/// Trait for returning references to default static values for a give types
pub trait DefaultStatic<T: Bitflag + Sized> {
    /// returns a reference to a default static value for a give type
    /// * example. &'static 0u8 for u8
    /// * example. &'static 0u16 for u16
    /// * .. u128
    fn default_static() -> &'static T;
}

impl DefaultStatic<u8> for u8 {
    fn default_static() -> &'static u8 {
        DEF_U8
    }
}

impl DefaultStatic<u16> for u16 {
    fn default_static() -> &'static u16 {
        DEF_U16
    }
}

impl DefaultStatic<u32> for u32 {
    fn default_static() -> &'static u32 {
        DEF_U32
    }
}

impl DefaultStatic<u64> for u64 {
    fn default_static() -> &'static u64 {
        DEF_U64
    }
}

impl DefaultStatic<u128> for u128 {
    fn default_static() -> &'static u128 {
        DEF_U128
    }
}

/// Iterator for iterating over each bit of each item in a Vec or Slice
///```
///  # use crate::cj_common::prelude::*;
/// let x = [2u64, 2, 2];
/// for i in x.as_slice().iter_to_bit().enumerate() {
///     match i.0 {
///         1 | 65 | 129 => assert_eq!(i.1, true),
///         _ => assert_eq!(i.1, false),
///     }
/// }
///```
pub struct BitStreamIter<'a, T>
where
    T: BitFlagIter<'a, T> + Bitflag + Sized,
{
    bit_count: usize,
    index: usize,
    stream: Iter<'a, T>,
    item: BitIter<'a, T>,
    is_done: bool,
}

impl<'a, T: BitFlagIter<'a, T> + Bitflag + Sized + DefaultStatic<T> + 'static>
    BitStreamIter<'a, T>
{
    pub fn new(mut iter: Iter<'a, T>, bit_count: usize) -> BitStreamIter<'a, T> {
        let x = iter.next();
        let is_none = x.is_none();

        BitStreamIter {
            bit_count,
            index: 0,
            stream: iter,
            item: x.unwrap_or(T::default_static()).bit_iter(),
            is_done: is_none,
        }
    }
    #[inline(always)]
    fn increment_item(&mut self) -> bool {
        if let Some(next) = self.stream.next() {
            self.index = 0;
            self.item = next.bit_iter();
        } else {
            self.is_done = true;
        }

        self.is_done
    }
    #[inline(always)]
    fn next_bit(&mut self) -> Option<bool> {
        if !self.is_done {
            if self.index >= self.bit_count {
                self.increment_item();
            }
            self.index += 1;
            self.item.next_bit()
        } else {
            None
        }
    }
}

impl<'a, T: BitFlagIter<'a, T> + Bitflag + Sized + DefaultStatic<T> + 'static> Iterator
    for BitStreamIter<'a, T>
{
    type Item = bool;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_bit()
    }
}

pub trait CjToBitStreamIter<'a, T: BitFlagIter<'a, T> + Bitflag + Sized> {
    /// returns a BitStreamIter for iterating over each bit of each item in a Vec or Slice
    fn iter_to_bit(&'a self) -> BitStreamIter<'a, T>;
}

impl<'a, T: BitFlagIter<'a, T> + Bitflag + DefaultStatic<T> + 'static> CjToBitStreamIter<'a, T>
    for Vec<T>
{
    #[inline]
    fn iter_to_bit(&'a self) -> BitStreamIter<'a, T> {
        let size = size_of::<T>() * 8;
        BitStreamIter::new(self[..].iter(), size)
    }
}

impl<'a, T: BitFlagIter<'a, T> + Bitflag + DefaultStatic<T> + 'static> CjToBitStreamIter<'a, T>
    for &[T]
{
    #[inline]
    fn iter_to_bit(&'a self) -> BitStreamIter<'a, T> {
        let size = size_of::<T>() * 8;
        BitStreamIter::new(self[..].iter(), size)
    }
}

pub trait CjMatchesMask<'a, T> {
    /// matches_mask performs bitwise '&(self & mask) == mask' to verify all bits in mask are matched in self.
    /// ```
    /// # use crate::cj_common::prelude::*;
    /// let mask = 0b00011010u8;
    /// let byte = 0b01011010u8;
    /// assert_eq!(byte.matches_mask(&mask),true);
    /// ```
    fn matches_mask(&self, mask: &T) -> bool;
    /// performs bitwise '&(self & value) == self' to verify all bits in self are matched in value
    /// ```
    /// # use crate::cj_common::prelude::*;
    /// let mask = 0b00011010u8;
    /// let byte = 0b01011010u8;
    /// assert_eq!(mask.as_mask_matches(&byte),true);
    /// ```
    fn as_mask_matches(&self, value: &T) -> bool;
}

impl<'a> CjMatchesMask<'a, u8> for u8 {
    #[inline]
    fn matches_mask(&self, mask: &u8) -> bool {
        &(self & mask) == mask
    }
    #[inline]
    fn as_mask_matches(&self, value: &u8) -> bool {
        &(self & value) == self
    }
}

impl<'a> CjMatchesMask<'a, u16> for u16 {
    #[inline]
    fn matches_mask(&self, mask: &u16) -> bool {
        &(self & mask) == mask
    }
    #[inline]
    fn as_mask_matches(&self, value: &u16) -> bool {
        &(self & value) == self
    }
}

impl<'a> CjMatchesMask<'a, u32> for u32 {
    #[inline]
    fn matches_mask(&self, mask: &u32) -> bool {
        &(self & mask) == mask
    }
    #[inline]
    fn as_mask_matches(&self, value: &u32) -> bool {
        &(self & value) == self
    }
}

impl<'a> CjMatchesMask<'a, u64> for u64 {
    #[inline]
    fn matches_mask(&self, mask: &u64) -> bool {
        &(self & mask) == mask
    }
    #[inline]
    fn as_mask_matches(&self, value: &u64) -> bool {
        &(self & value) == self
    }
}

impl<'a> CjMatchesMask<'a, u128> for u128 {
    #[inline]
    fn matches_mask(&self, mask: &u128) -> bool {
        &(self & mask) == mask
    }
    #[inline]
    fn as_mask_matches(&self, value: &u128) -> bool {
        &(self & value) == self
    }
}

#[cfg(test)]
mod test {
    use crate::cj_binary::bitbuf::{BitFlagIter, Bitflag};
    use crate::prelude::CjToBitStreamIter;

    #[test]
    fn test_u8_get() {
        let x = 0xABu8;
        assert!(x.get_bit(0));
        assert!(x.get_bit(1));
        assert!(!x.get_bit(2));
        assert!(!x.get_bit(100));
    }

    #[test]
    fn test_u8_set() {
        let mut x = 0x00u8;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert!(x.get_bit(0));

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert!(x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));
    }

    #[test]
    fn test_u16_get() {
        let x = 0xABu16;
        assert!(x.get_bit(0));
        assert!(x.get_bit(1));
        assert!(!x.get_bit(2));
        assert!(!x.get_bit(100));
    }

    #[test]
    fn test_u16_set() {
        let mut x = 0x00u16;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert!(x.get_bit(0));

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert!(x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));
    }

    #[test]
    fn test_u32_get() {
        let x = 0xABu32;
        assert!(x.get_bit(0));
        assert!(x.get_bit(1));
        assert!(!x.get_bit(2));
        assert!(!x.get_bit(100));
    }

    #[test]
    fn test_u32_set() {
        let mut x = 0x00u32;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert!(x.get_bit(0));

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert!(x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));
    }

    #[test]
    fn test_u64_get() {
        let x = 0xABu64;
        assert!(x.get_bit(0));
        assert!(x.get_bit(1));
        assert!(!x.get_bit(2));
        assert!(!x.get_bit(100));
    }

    #[test]
    fn test_u64_set() {
        let mut x = 0x00u64;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert!(x.get_bit(0));

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert!(x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));
    }

    #[test]
    fn test_u128_get() {
        let x = 0xABu128;
        assert!(x.get_bit(0));
        assert!(x.get_bit(1));
        assert!(!x.get_bit(2));
        assert!(!x.get_bit(100));
    }

    #[test]
    fn test_u128_set() {
        let mut x = 0x00u128;

        x.set_bit(0, true);
        assert_eq!(x, 1);
        assert!(x.get_bit(0));

        x.set_bit(1, true);
        assert_eq!(x, 3);
        assert!(x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));

        x.set_bit(1, false);
        assert_eq!(x, 1);
        assert!(!x.get_bit(1));
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

    #[test]
    fn test_vec_u8_iter() {
        let x = vec![0xABu8, 0xAB, 0xAB];
        let mut v = Vec::new();
        for i in x.iter_to_bit() {
            v.push(i);
        }

        assert_eq!(
            v.as_slice(),
            &[
                true, true, false, true, false, true, false, true, true, true, false, true, false,
                true, false, true, true, true, false, true, false, true, false, true
            ]
        );
    }

    #[test]
    fn test_vec_u16_iter() {
        let x = vec![0xAB00u16, 0xAB00, 0xAB00];
        let mut v = Vec::new();
        for i in x.iter_to_bit() {
            v.push(i);
        }

        assert_eq!(
            v.as_slice(),
            &[
                false, false, false, false, false, false, false, false, true, true, false, true,
                false, true, false, true, false, false, false, false, false, false, false, false,
                true, true, false, true, false, true, false, true, false, false, false, false,
                false, false, false, false, true, true, false, true, false, true, false, true,
            ]
        );
    }

    #[test]
    fn test_vec_u32_iter() {
        let x = vec![2u32, 2, 2];
        for i in x.iter_to_bit().enumerate() {
            match i.0 {
                1 | 33 | 65 => assert!(i.1),
                _ => assert!(!i.1),
            }
        }
    }

    #[test]
    fn test_vec_u64_iter() {
        let x = vec![2u64, 2, 2];
        for i in x.iter_to_bit().enumerate() {
            match i.0 {
                1 | 65 | 129 => assert!(i.1),
                _ => assert!(!i.1),
            }
        }
    }

    #[test]
    fn test_vec_u128_iter() {
        let x = vec![2u128, 2, 2];
        for i in x.iter_to_bit().enumerate() {
            match i.0 {
                1 | 129 | 257 => assert!(i.1),
                _ => assert!(!i.1),
            }
        }
    }

    #[test]
    fn test_u8_slice_iter() {
        let x = [0xABu8, 0xAB, 0xAB];
        let mut v = Vec::new();
        for i in x.as_slice().iter_to_bit() {
            v.push(i);
        }

        assert_eq!(
            v.as_slice(),
            &[
                true, true, false, true, false, true, false, true, true, true, false, true, false,
                true, false, true, true, true, false, true, false, true, false, true
            ]
        );
    }

    #[test]
    fn test_u16_slice_iter() {
        let x = [0xAB00u16, 0xAB00, 0xAB00];
        let mut v = Vec::new();
        for i in x.as_slice().iter_to_bit() {
            v.push(i);
        }

        assert_eq!(
            v.as_slice(),
            &[
                false, false, false, false, false, false, false, false, true, true, false, true,
                false, true, false, true, false, false, false, false, false, false, false, false,
                true, true, false, true, false, true, false, true, false, false, false, false,
                false, false, false, false, true, true, false, true, false, true, false, true,
            ]
        );
    }

    #[test]
    fn test_u32_slice_iter() {
        let x = [2u32, 2, 2];
        for i in x.as_slice().iter_to_bit().enumerate() {
            match i.0 {
                1 | 33 | 65 => assert!(i.1),
                _ => assert!(!i.1),
            }
        }
    }

    #[test]
    fn test_u64_slice_iter() {
        let x = [2u64, 2, 2];
        for i in x.as_slice().iter_to_bit().enumerate() {
            match i.0 {
                1 | 65 | 129 => assert!(i.1),
                _ => assert!(!i.1),
            }
        }
    }

    #[test]
    fn test_u128_slice_iter() {
        let x = [2u128, 2, 2];
        for i in x.as_slice().iter_to_bit().enumerate() {
            match i.0 {
                1 | 129 | 257 => assert!(i.1),
                _ => assert!(!i.1),
            }
        }
    }

    #[test]
    fn test_u8_vec_iter() {
        // vec of 256 u8 values from 0 to 255
        let x = (0..256).map(|x| x as u8).collect::<Vec<u8>>();
        let mut v = Vec::with_capacity(256);
        for i in x.iter_to_bit() {
            v.push(i);
        }

        assert_eq!(
            v.as_slice(),
            &[
                false, false, false, false, false, false, false, false, true, false, false, false,
                false, false, false, false, false, true, false, false, false, false, false, false,
                true, true, false, false, false, false, false, false, false, false, true, false,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, true, true, false, false, false, false, false, true, true, true, false,
                false, false, false, false, false, false, false, true, false, false, false, false,
                true, false, false, true, false, false, false, false, false, true, false, true,
                false, false, false, false, true, true, false, true, false, false, false, false,
                false, false, true, true, false, false, false, false, true, false, true, true,
                false, false, false, false, false, true, true, true, false, false, false, false,
                true, true, true, true, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, false, false, true, false, false, false,
                false, true, false, false, true, false, false, false, true, true, false, false,
                true, false, false, false, false, false, true, false, true, false, false, false,
                true, false, true, false, true, false, false, false, false, true, true, false,
                true, false, false, false, true, true, true, false, true, false, false, false,
                false, false, false, true, true, false, false, false, true, false, false, true,
                true, false, false, false, false, true, false, true, true, false, false, false,
                true, true, false, true, true, false, false, false, false, false, true, true, true,
                false, false, false, true, false, true, true, true, false, false, false, false,
                true, true, true, true, false, false, false, true, true, true, true, true, false,
                false, false, false, false, false, false, false, true, false, false, true, false,
                false, false, false, true, false, false, false, true, false, false, false, true,
                false, false, true, true, false, false, false, true, false, false, false, false,
                true, false, false, true, false, false, true, false, true, false, false, true,
                false, false, false, true, true, false, false, true, false, false, true, true,
                true, false, false, true, false, false, false, false, false, true, false, true,
                false, false, true, false, false, true, false, true, false, false, false, true,
                false, true, false, true, false, false, true, true, false, true, false, true,
                false, false, false, false, true, true, false, true, false, false, true, false,
                true, true, false, true, false, false, false, true, true, true, false, true, false,
                false, true, true, true, true, false, true, false, false, false, false, false,
                false, true, true, false, false, true, false, false, false, true, true, false,
                false, false, true, false, false, true, true, false, false, true, true, false,
                false, true, true, false, false, false, false, true, false, true, true, false,
                false, true, false, true, false, true, true, false, false, false, true, true,
                false, true, true, false, false, true, true, true, false, true, true, false, false,
                false, false, false, true, true, true, false, false, true, false, false, true,
                true, true, false, false, false, true, false, true, true, true, false, false, true,
                true, false, true, true, true, false, false, false, false, true, true, true, true,
                false, false, true, false, true, true, true, true, false, false, false, true, true,
                true, true, true, false, false, true, true, true, true, true, true, false, false,
                false, false, false, false, false, false, true, false, true, false, false, false,
                false, false, true, false, false, true, false, false, false, false, true, false,
                true, true, false, false, false, false, true, false, false, false, true, false,
                false, false, true, false, true, false, true, false, false, false, true, false,
                false, true, true, false, false, false, true, false, true, true, true, false,
                false, false, true, false, false, false, false, true, false, false, true, false,
                true, false, false, true, false, false, true, false, false, true, false, true,
                false, false, true, false, true, true, false, true, false, false, true, false,
                false, false, true, true, false, false, true, false, true, false, true, true,
                false, false, true, false, false, true, true, true, false, false, true, false,
                true, true, true, true, false, false, true, false, false, false, false, false,
                true, false, true, false, true, false, false, false, true, false, true, false,
                false, true, false, false, true, false, true, false, true, true, false, false,
                true, false, true, false, false, false, true, false, true, false, true, false,
                true, false, true, false, true, false, true, false, false, true, true, false, true,
                false, true, false, true, true, true, false, true, false, true, false, false,
                false, false, true, true, false, true, false, true, false, false, true, true,
                false, true, false, false, true, false, true, true, false, true, false, true, true,
                false, true, true, false, true, false, false, false, true, true, true, false, true,
                false, true, false, true, true, true, false, true, false, false, true, true, true,
                true, false, true, false, true, true, true, true, true, false, true, false, false,
                false, false, false, false, true, true, false, true, false, false, false, false,
                true, true, false, false, true, false, false, false, true, true, false, true, true,
                false, false, false, true, true, false, false, false, true, false, false, true,
                true, false, true, false, true, false, false, true, true, false, false, true, true,
                false, false, true, true, false, true, true, true, false, false, true, true, false,
                false, false, false, true, false, true, true, false, true, false, false, true,
                false, true, true, false, false, true, false, true, false, true, true, false, true,
                true, false, true, false, true, true, false, false, false, true, true, false, true,
                true, false, true, false, true, true, false, true, true, false, false, true, true,
                true, false, true, true, false, true, true, true, true, false, true, true, false,
                false, false, false, false, true, true, true, false, true, false, false, false,
                true, true, true, false, false, true, false, false, true, true, true, false, true,
                true, false, false, true, true, true, false, false, false, true, false, true, true,
                true, false, true, false, true, false, true, true, true, false, false, true, true,
                false, true, true, true, false, true, true, true, false, true, true, true, false,
                false, false, false, true, true, true, true, false, true, false, false, true, true,
                true, true, false, false, true, false, true, true, true, true, false, true, true,
                false, true, true, true, true, false, false, false, true, true, true, true, true,
                false, true, false, true, true, true, true, true, false, false, true, true, true,
                true, true, true, false, true, true, true, true, true, true, true, false, false,
                false, false, false, false, false, false, true, true, false, false, false, false,
                false, false, true, false, true, false, false, false, false, false, true, true,
                true, false, false, false, false, false, true, false, false, true, false, false,
                false, false, true, true, false, true, false, false, false, false, true, false,
                true, true, false, false, false, false, true, true, true, true, false, false,
                false, false, true, false, false, false, true, false, false, false, true, true,
                false, false, true, false, false, false, true, false, true, false, true, false,
                false, false, true, true, true, false, true, false, false, false, true, false,
                false, true, true, false, false, false, true, true, false, true, true, false,
                false, false, true, false, true, true, true, false, false, false, true, true, true,
                true, true, false, false, false, true, false, false, false, false, true, false,
                false, true, true, false, false, false, true, false, false, true, false, true,
                false, false, true, false, false, true, true, true, false, false, true, false,
                false, true, false, false, true, false, true, false, false, true, true, false,
                true, false, true, false, false, true, false, true, true, false, true, false,
                false, true, true, true, true, false, true, false, false, true, false, false,
                false, true, true, false, false, true, true, false, false, true, true, false,
                false, true, false, true, false, true, true, false, false, true, true, true, false,
                true, true, false, false, true, false, false, true, true, true, false, false, true,
                true, false, true, true, true, false, false, true, false, true, true, true, true,
                false, false, true, true, true, true, true, true, false, false, true, false, false,
                false, false, false, true, false, true, true, false, false, false, false, true,
                false, true, false, true, false, false, false, true, false, true, true, true,
                false, false, false, true, false, true, false, false, true, false, false, true,
                false, true, true, false, true, false, false, true, false, true, false, true, true,
                false, false, true, false, true, true, true, true, false, false, true, false, true,
                false, false, false, true, false, true, false, true, true, false, false, true,
                false, true, false, true, false, true, false, true, false, true, false, true, true,
                true, false, true, false, true, false, true, false, false, true, true, false, true,
                false, true, true, false, true, true, false, true, false, true, false, true, true,
                true, false, true, false, true, true, true, true, true, false, true, false, true,
                false, false, false, false, true, true, false, true, true, false, false, false,
                true, true, false, true, false, true, false, false, true, true, false, true, true,
                true, false, false, true, true, false, true, false, false, true, false, true, true,
                false, true, true, false, true, false, true, true, false, true, false, true, true,
                false, true, true, false, true, true, true, true, false, true, true, false, true,
                false, false, false, true, true, true, false, true, true, false, false, true, true,
                true, false, true, false, true, false, true, true, true, false, true, true, true,
                false, true, true, true, false, true, false, false, true, true, true, true, false,
                true, true, false, true, true, true, true, false, true, false, true, true, true,
                true, true, false, true, true, true, true, true, true, true, false, true, false,
                false, false, false, false, false, true, true, true, false, false, false, false,
                false, true, true, false, true, false, false, false, false, true, true, true, true,
                false, false, false, false, true, true, false, false, true, false, false, false,
                true, true, true, false, true, false, false, false, true, true, false, true, true,
                false, false, false, true, true, true, true, true, false, false, false, true, true,
                false, false, false, true, false, false, true, true, true, false, false, true,
                false, false, true, true, false, true, false, true, false, false, true, true, true,
                true, false, true, false, false, true, true, false, false, true, true, false,
                false, true, true, true, false, true, true, false, false, true, true, false, true,
                true, true, false, false, true, true, true, true, true, true, false, false, true,
                true, false, false, false, false, true, false, true, true, true, false, false,
                false, true, false, true, true, false, true, false, false, true, false, true, true,
                true, true, false, false, true, false, true, true, false, false, true, false, true,
                false, true, true, true, false, true, false, true, false, true, true, false, true,
                true, false, true, false, true, true, true, true, true, false, true, false, true,
                true, false, false, false, true, true, false, true, true, true, false, false, true,
                true, false, true, true, false, true, false, true, true, false, true, true, true,
                true, false, true, true, false, true, true, false, false, true, true, true, false,
                true, true, true, false, true, true, true, false, true, true, false, true, true,
                true, true, false, true, true, true, true, true, true, true, false, true, true,
                false, false, false, false, false, true, true, true, true, false, false, false,
                false, true, true, true, false, true, false, false, false, true, true, true, true,
                true, false, false, false, true, true, true, false, false, true, false, false,
                true, true, true, true, false, true, false, false, true, true, true, false, true,
                true, false, false, true, true, true, true, true, true, false, false, true, true,
                true, false, false, false, true, false, true, true, true, true, false, false, true,
                false, true, true, true, false, true, false, true, false, true, true, true, true,
                true, false, true, false, true, true, true, false, false, true, true, false, true,
                true, true, true, false, true, true, false, true, true, true, false, true, true,
                true, false, true, true, true, true, true, true, true, false, true, true, true,
                false, false, false, false, true, true, true, true, true, false, false, false,
                true, true, true, true, false, true, false, false, true, true, true, true, true,
                true, false, false, true, true, true, true, false, false, true, false, true, true,
                true, true, true, false, true, false, true, true, true, true, false, true, true,
                false, true, true, true, true, true, true, true, false, true, true, true, true,
                false, false, false, true, true, true, true, true, true, false, false, true, true,
                true, true, true, false, true, false, true, true, true, true, true, true, true,
                false, true, true, true, true, true, false, false, true, true, true, true, true,
                true, true, false, true, true, true, true, true, true, false, true, true, true,
                true, true, true, true, true, true, true, true, true, true, true, true
            ]
        )
    }
}
