pub mod in_set {
    use std::ops::{Range, RangeInclusive};

    pub trait CjInSet<T> {
        fn in_set(&self, values: T) -> bool;
    }

    impl<T> CjInSet<Range<T>> for T
    where
        T: PartialEq + PartialOrd,
    {
        /// returns true if a value is in a range
        /// ```
        /// # use cj_common::prelude::CjInSet;
        /// assert_eq!(9u8.in_set(0..12), true);
        /// assert_eq!('x'.in_set('s'..'z'), true);    
        /// ```
        fn in_set(&self, values: Range<T>) -> bool {
            values.contains(self)
        }
    }

    impl<T> CjInSet<RangeInclusive<T>> for T
    where
        T: PartialEq + PartialOrd,
    {
        /// returns true if a value is in an inclusive range
        /// ```
        /// # use cj_common::prelude::CjInSet;
        /// assert_eq!(9u8.in_set(0..=9), true);
        /// assert_eq!('z'.in_set('s'..='z'), true);    
        /// ```
        fn in_set(&self, values: RangeInclusive<T>) -> bool {
            values.contains(self)
        }
    }

    impl<T> CjInSet<&[Range<T>]> for T
    where
        T: PartialEq + PartialOrd,
    {
        /// returns true if a value is in a slice of ranges
        /// ```
        /// # use cj_common::prelude::CjInSet;
        /// # use std::ops::Range;
        /// assert_eq!(9u8.in_set([0..6, 5..12].as_slice()), true);
        /// assert_eq!('x'.in_set(['q'..'t', 's'..'z'].as_slice()), true);
        ///
        /// const ALPHA_NUM: &[Range<char>] = ['a'..'z', 'A'..'Z', '0'..'9'].as_slice();         
        /// assert_eq!('x'.in_set(ALPHA_NUM), true);
        ///
        /// ```
        fn in_set(&self, values: &[Range<T>]) -> bool {
            for vals in values {
                if vals.contains(self) {
                    return true;
                }
            }
            false
        }
    }

    impl<T> CjInSet<&[RangeInclusive<T>]> for T
    where
        T: PartialEq + PartialOrd,
    {
        /// returns true if a value is in a slice of ranges
        /// ```
        /// # use cj_common::prelude::CjInSet;
        /// # use std::ops::RangeInclusive;
        /// assert_eq!(9u8.in_set([0..=6, 5..=12].as_slice()), true);
        /// assert_eq!('x'.in_set(['q'..='t', 's'..='z'].as_slice()), true);
        ///
        /// const ALPHA_NUM: &[RangeInclusive<char>] = ['a'..='z', 'A'..='Z', '0'..='9'].as_slice();         
        /// assert_eq!('x'.in_set(ALPHA_NUM), true);
        ///
        /// ```
        fn in_set(&self, values: &[RangeInclusive<T>]) -> bool {
            for vals in values {
                if vals.contains(self) {
                    return true;
                }
            }
            false
        }
    }

    impl<T> CjInSet<&[T]> for T
    where
        T: PartialEq + PartialOrd,
    {
        /// returns true if a value is in a slice
        /// ```
        /// # use cj_common::prelude::CjInSet;       
        /// assert_eq!(9u8.in_set([0,1,9,212].as_slice()), true);
        /// assert_eq!('x'.in_set(['a','s','x','z'].as_slice()), true);    
        /// ```
        fn in_set(&self, values: &[T]) -> bool {
            values.contains(self)
        }
    }

    /// this trait is just a helper type to distinguish chars from other types
    pub trait CjChair {
        /// returns a ref to itself. Used as a helper function to coerce char from T where T is a char
        fn as_char(&self) -> &char;
    }

    impl CjChair for char {
        #[inline]
        fn as_char(&self) -> &char {
            &self
        }
    }

    impl<T> CjInSet<&str> for T
    where
        T: PartialEq + PartialOrd + CjChair,
    {
        /// returns true if a char is in a str slice
        /// ```
        /// # use cj_common::prelude::CjInSet;
        /// assert_eq!('x'.in_set("abcxyz"), true);
        /// ```
        fn in_set(&self, values: &str) -> bool {
            for c in values.chars() {
                if &c == self.as_char() {
                    return true;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::in_set::*;

    #[test]
    fn test_u8_in_set() {
        assert_eq!(9u8.in_set(5..12), true);
    }

    #[test]
    fn test_u8_in_set2() {
        assert_eq!(9u8.in_set([0..6, 5..12].as_slice()), true);
    }

    #[test]
    fn test_char_in_set() {
        assert_eq!('r'.in_set('q'..'t'), true);
    }

    #[test]
    fn test_char_in_set2() {
        assert_eq!('x'.in_set(['q'..'t', 's'..'z'].as_slice()), true);
    }

    #[test]
    fn test_char_in_set3() {
        const ALPHA_NUM: &[RangeInclusive<char>] = ['a'..='z', 'A'..='Z', '0'..='9'].as_slice();
        assert_eq!('z'.in_set(ALPHA_NUM), true);
    }

    #[test]
    fn test_char_in_set4() {
        assert_eq!('z'.in_set("abcxyz"), true);
    }

    #[test]
    fn test_from_readme() {
        assert_eq!(11u8.in_set(5..12), true);
        assert_eq!(12u8.in_set(5..=12), true);

        assert_eq!(9u8.in_set([0..6, 5..12].as_slice()), true);
        assert_eq!(12u8.in_set([0..=6, 5..=12].as_slice()), true);

        assert_eq!('x'.in_set(['q'..'t', 's'..'z'].as_slice()), true);
        assert_eq!('z'.in_set(['q'..='t', 's'..='z'].as_slice()), true);

        const ALPHA_NUM: &[RangeInclusive<char>] = ['a'..='z', 'A'..='Z', '0'..='9'].as_slice();
        assert_eq!('x'.in_set(ALPHA_NUM), true);

        assert_eq!('x'.in_set("abcxyz"), true);

        assert_eq!(9u8.in_set([0, 1, 9, 212].as_slice()), true);
        assert_eq!('x'.in_set(['a', 's', 'x', 'z'].as_slice()), true);
    }
}
