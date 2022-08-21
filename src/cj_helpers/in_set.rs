pub mod in_set {
    use std::ops::Range;

    pub trait CjInSet<T> {
        fn in_set(&self, values: T) -> bool;
    }

    impl<T> CjInSet<Range<T>> for T
    where
        T: PartialEq + PartialOrd,
    {
        fn in_set(&self, values: Range<T>) -> bool {
            values.contains(self)
        }
    }

    impl<T> CjInSet<&[Range<T>]> for T
    where
        T: PartialEq + PartialOrd,
    {
        fn in_set(&self, values: &[Range<T>]) -> bool {
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
        fn in_set(&self, values: &[T]) -> bool {
            values.contains(self)
        }
    }

    pub trait CjChair {
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
    use std::ops::Range;

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
        const ALPHA_NUM: &[Range<char>] = ['a'..'z', 'A'..'Z', '0'..'9'].as_slice();
        assert_eq!('x'.in_set(ALPHA_NUM), true);
    }

    #[test]
    fn test_char_in_set4() {
        assert_eq!('x'.in_set("abcxyz"), true);
    }
}
