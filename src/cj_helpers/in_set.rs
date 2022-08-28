pub mod in_set {

    use std::ops::{Range, RangeInclusive};

    trait CjExactRng<T: PartialOrd + PartialEq> {
        fn in_exact_range(&self, value: &T) -> bool;
    }

    impl<T: PartialOrd + PartialEq> CjExactRng<T> for Range<T> {
        #[inline]
        fn in_exact_range(&self, value: &T) -> bool {
            self.contains(value)
        }
    }

    impl<T: PartialOrd + PartialEq> CjExactRng<T> for RangeInclusive<T> {
        #[inline]
        fn in_exact_range(&self, value: &T) -> bool {
            self.contains(value)
        }
    }

    impl<'a, T: PartialOrd + PartialEq> CjExactRng<T> for CjExactRange<'a, T> {
        #[inline]
        fn in_exact_range(&self, value: &T) -> bool {
            match &self.inner {
                RangeType::Exact(s, e) => (s <= value) && (e >= value),
                RangeType::Slice(s) => s.contains(value),
                _ => false,
            }
        }
    }

    trait CjCharsOnly: CjChar + PartialOrd + PartialEq {}

    impl<'a, T: CjCharsOnly> CjExactRng<T> for CjExactRange<'a, char> {
        #[inline]
        fn in_exact_range(&self, value: &T) -> bool {
            match &self.inner {
                RangeType::Exact(s, e) => (s <= value.as_char()) && (e >= value.as_char()),
                RangeType::Slice(s) => s.contains(value.as_char()),
                RangeType::Str(s) => {
                    for c in s.chars() {
                        if value.as_char() == &c {
                            return true;
                        }
                    }
                    false
                }
            }
        }
    }

    impl<'a, T> Into<CjExactRange<'a, T>> for Range<T> {
        #[inline]
        fn into(self) -> CjExactRange<'a, T> {
            CjExactRange {
                inner: RangeType::Exact(self.start, self.end),
            }
        }
    }

    impl<'a, T> Into<CjExactRange<'a, T>> for RangeInclusive<T> {
        #[inline]
        fn into(self) -> CjExactRange<'a, T> {
            let (s, e) = self.into_inner();
            CjExactRange {
                inner: RangeType::Exact(s, e),
            }
        }
    }

    impl<'a, T> Into<CjExactRange<'a, T>> for &'a [T] {
        #[inline]
        fn into(self) -> CjExactRange<'a, T> {
            CjExactRange {
                inner: RangeType::Slice(self),
            }
        }
    }

    impl<'a, T: CjChar> Into<CjExactRange<'a, T>> for &'a str {
        #[inline]
        fn into(self) -> CjExactRange<'a, T> {
            CjExactRange {
                inner: RangeType::Str(self),
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum RangeType<'a, T> {
        Exact(T, T),
        Slice(&'a [T]),
        Str(&'a str),
    }

    /// CjExactRange is similar to RangeInclusive and is used by the in_set() method.
    /// in_set() requires CjExactRange in order to support a mixed slice of Range, RangeInclusive, Slice and str.
    /// Note that Range<T>.into(), RangeInclusive<T>.into(), Slice<T>.into() and &str.into() have been implemented
    /// for CjExactRange for easy conversion.
    ///
    /// is_set is auto implemented for all types it supports.
    /// ```
    /// # use cj_common::prelude::CjInSets;
    /// assert_eq!(
    ///    'z'.in_set(
    ///         [
    ///             ('a'..'r').into(),               // Range into CjExactRange
    ///             ('r'..='z').into(),              // RangeInclusive into CjExactRange
    ///             ['a','b','c'].as_slice().into(), // Slice into CjExactRange
    ///             "test123".into(),                // str into CjExactRange
    ///         ].as_slice()
    ///     ),
    ///     true
    /// );
    /// ```
    /// char example:
    /// ```
    ///  # use cj_common::prelude::CjInSets;
    /// let list = "lmnop";
    /// for c in list.chars() {
    ///     assert_eq!(
    ///        c.in_set(
    ///             [
    ///                 ('k'..='l').into(),                // RangeInclusive
    ///                 ('m'..'n').into(),                 // Range
    ///                 ('n'..='p').into(),                // RangeInclusive
    ///                 ['a', 'b', 'c'].as_slice().into(), // Slice
    ///                 "test123".into(),                  // str
    ///             ]
    ///             .as_slice()
    ///         ),
    ///         true
    ///     );
    /// }
    /// ```
    /// i32 example:
    /// ```
    ///  # use cj_common::prelude::CjInSets;
    /// let list = [1_000, 10_000, 100_000_000];
    /// for n in list {
    ///     assert_eq!(
    ///         n.in_set(
    ///            [
    ///                 (1..=10).into(),                 // RangeInclusive
    ///                 (500..2_000).into(),             // Range
    ///                 (9_999..=100_000_000).into(),    // RangeInclusive
    ///                 [30, 90, 700].as_slice().into()  // Slice
    ///             ]
    ///             .as_slice()
    ///         ),
    ///         true
    ///     );
    /// }        
    /// ```
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct CjExactRange<'a, T> {
        inner: RangeType<'a, T>,
    }

    trait CjInExactSet<T: PartialEq + PartialOrd> {
        fn in_exact_set(&self, value: &T) -> bool;
    }

    impl<'a, T: PartialEq + PartialOrd> CjInExactSet<T> for CjExactRange<'a, T> {
        #[inline]
        fn in_exact_set(&self, value: &T) -> bool {
            self.in_exact_range(value)
        }
    }

    impl<'a, T: PartialEq + PartialOrd> CjInExactSet<T> for &[CjExactRange<'a, T>] {
        #[inline]
        fn in_exact_set(&self, value: &T) -> bool {
            for v in self.iter() {
                if v.in_exact_range(value) {
                    return true;
                }
            }
            false
        }
    }

    pub trait CjInRange<T: PartialOrd + PartialEq> {
        /// Returns true if a value is in a given Range
        /// ```
        /// # use cj_common::prelude::CjInRange;
        /// assert_eq!('x'.in_range('a'..'z'), true);
        /// ```
        fn in_range(&self, value: Range<T>) -> bool;
    }

    impl<T: PartialEq + PartialOrd> CjInRange<T> for T {
        #[inline]
        fn in_range(&self, value: Range<T>) -> bool {
            value.contains(self)
        }
    }

    pub trait CjInRangeInclusive<T: PartialOrd + PartialEq> {
        /// Returns true if a value is in a given Range
        /// ```
        /// # use cj_common::prelude::CjInRangeInclusive;
        /// assert_eq!('x'.in_range_inclusive('a'..='z'), true);
        /// ```
        fn in_range_inclusive(&self, value: RangeInclusive<T>) -> bool;
    }

    impl<T: PartialEq + PartialOrd> CjInRangeInclusive<T> for T {
        #[inline]
        fn in_range_inclusive(&self, value: RangeInclusive<T>) -> bool {
            value.contains(self)
        }
    }

    pub trait CjInSets<T: PartialEq + PartialOrd> {
        /// Returns true if a value is within a give slice of ranges.
        /// Note that this method requires ranges to be of type CjExactRange,
        /// so Range<T>.into(), RangeInclusive<T>.into(), Slice<T>.into() and &str.into() have been implemented
        /// for CjExactRange for easy conversion.
        ///
        /// is_set is auto implemented for all types it supports.
        /// ```
        /// # use cj_common::prelude::CjInSets;
        /// assert_eq!(
        ///    'z'.in_set(
        ///         [
        ///             ('a'..'r').into(),               // Range
        ///             ('r'..='z').into(),              // RangeInclusive
        ///             ['a','b','c'].as_slice().into(), // Slice
        ///             "test123".into(),                // str
        ///         ].as_slice()
        ///     ),
        ///     true
        /// );
        /// ```
        /// char example:
        /// ```
        ///  # use cj_common::prelude::CjInSets;
        /// let list = "lmnop";
        /// for c in list.chars() {
        ///     assert_eq!(
        ///        c.in_set(
        ///             [
        ///                 ('k'..='l').into(),                // RangeInclusive
        ///                 ('m'..'n').into(),                 // Range
        ///                 ('n'..='p').into(),                // RangeInclusive
        ///                 ['a', 'b', 'c'].as_slice().into(), // Slice
        ///                 "test123".into(),                  // str
        ///             ]
        ///             .as_slice()
        ///         ),
        ///         true
        ///     );
        /// }
        /// ```
        /// i32 example:
        /// ```
        ///  # use cj_common::prelude::CjInSets;
        /// let list = [1_000, 10_000, 100_000_000];
        /// for n in list {
        ///     assert_eq!(
        ///         n.in_set(
        ///            [
        ///                 (1..=10).into(),                 // RangeInclusive
        ///                 (500..2_000).into(),             // Range
        ///                 (9_999..=100_000_000).into(),    // RangeInclusive
        ///                 [30, 90, 700].as_slice().into()  // Slice
        ///             ]
        ///             .as_slice()
        ///         ),
        ///         true
        ///     );
        /// }        
        /// ```
        fn in_set(&self, value: &[CjExactRange<T>]) -> bool;
    }

    impl<T: PartialEq + PartialOrd> CjInSets<T> for T {
        #[inline]
        fn in_set(&self, value: &[CjExactRange<T>]) -> bool {
            value.in_exact_set(self)
        }
    }

    pub trait CjChar {
        /// returns a ref to itself. Used as a helper function to coerce char from T where T is a char
        fn as_char(&self) -> &char;
    }

    impl CjChar for char {
        #[inline]
        fn as_char(&self) -> &char {
            &self
        }
    }

    pub trait CjIsAscii {
        /// returns true if char is in ['a'..='z', 'A'..='Z']
        fn is_ascii_alpha(&self) -> bool;
        /// returns true if char is in ['0'..='9']
        fn is_ascii_numeric(&self) -> bool;
        /// returns true if char is in ['a'..='z', 'A'..='Z', '0'..='9']
        fn is_ascii_alpha_numeric(&self) -> bool;
    }

    impl CjIsAscii for char {
        fn is_ascii_alpha(&self) -> bool {
            self.in_set([('a'..='z').into(), ('A'..='Z').into()].as_slice())
        }

        fn is_ascii_numeric(&self) -> bool {
            self.in_range_inclusive('0'..='9')
        }

        fn is_ascii_alpha_numeric(&self) -> bool {
            self.in_set([('a'..='z').into(), ('A'..='Z').into(), ('0'..='9').into()].as_slice())
        }
    }

    #[cfg(test)]
    pub mod test {
        use super::*;

        #[test]
        fn test_inset_1() {
            assert_eq!('x'.in_range('a'..'z'), true);
            assert_eq!('z'.in_range_inclusive('a'..='z'), true);
            assert_eq!(1.in_range(1..3), true);
        }

        #[test]
        fn test_inset_1b() {
            assert_eq!('x'.in_range_inclusive('a'..='z'), true);
            assert_eq!('z'.in_range_inclusive('a'..='z'), true);
            assert_eq!(1.in_range_inclusive(1..=3), true);
        }

        #[test]
        fn test_inset_2() {
            assert_eq!(
                'x'.in_set([('a'..'r').into(), ('r'..'y').into()].as_slice()),
                true
            );
            assert_eq!(
                'z'.in_set([('a'..'r').into(), ('r'..='z').into()].as_slice()),
                true
            );
            assert_eq!(10.in_set([(1..3).into(), (3..=10).into()].as_slice()), true);
        }

        #[test]
        fn test_inset_3() {
            let list = "lmnop";
            for c in list.chars() {
                assert_eq!(c.in_range('k'..'q'), true);
                assert_eq!(c.in_set([('k'..'q').into()].as_slice()), true);
                assert_eq!(
                    c.in_set(
                        [('k'..='l').into(), ('m'..'n').into(), ('n'..='p').into()].as_slice()
                    ),
                    true
                );
                assert_eq!(c.in_range('w'..'z'), false);
            }
        }

        #[test]
        fn test_inset_4() {
            let list = [1_000, 10_000, 100_000_000];
            for n in list {
                assert_eq!(n.in_range(1..200_000_000), true);
                assert_eq!(n.in_set([(1..200_000_000).into()].as_slice()), true);
                assert_eq!(
                    n.in_set(
                        [
                            (1..=10).into(),
                            (500..2_000).into(),
                            (9_999..=100_000_000).into(),
                        ]
                        .as_slice()
                    ),
                    true
                );
                assert_eq!(n.in_range(1_000_000_000..1_000_000_001), false);
            }
        }

        #[test]
        fn test_inset_5() {
            let alpha_nums = [('a'..='z').into(), ('A'..='Z').into(), ('0'..='9').into()];
            let list = "lmnop";
            for c in list.chars() {
                assert_eq!(c.in_set(alpha_nums.as_slice()), true);
            }
        }

        #[test]
        fn test_inset_readme() {
            assert_eq!('x'.in_range('a'..'z'), true);
            assert_eq!('z'.in_range_inclusive('a'..='z'), true);
            assert_eq!(1.in_range(1..3), true);
            assert_eq!(
                'z'.in_set([('a'..'r').into(), ('r'..='z').into()].as_slice()),
                true
            );

            let list = "lmnop";
            for c in list.chars() {
                assert_eq!(c.in_range('k'..'q'), true);
                assert_eq!(c.in_set([('k'..'q').into()].as_slice()), true);
                assert_eq!(
                    c.in_set(
                        [
                            ('k'..='l').into(),                // RangeInclusive
                            ('m'..'n').into(),                 // Range
                            ('n'..='p').into(),                // RangeInclusive
                            ['a', 'b', 'c'].as_slice().into(), // Slice
                            "test123".into(),                  // str
                        ]
                        .as_slice()
                    ),
                    true
                );
                assert_eq!(c.in_range('w'..'z'), false);
            }

            let alpha_nums = [('a'..='z').into(), ('A'..='Z').into(), ('0'..='9').into()];
            let list = "lmnop";
            for c in list.chars() {
                assert_eq!(c.in_set(alpha_nums.as_slice()), true);
            }

            let list = [1_000, 10_000, 100_000_000];
            for n in list {
                assert_eq!(n.in_range(1..200_000_000), true);
                assert_eq!(n.in_set([(1..200_000_000).into()].as_slice()), true);
                assert_eq!(
                    n.in_set(
                        [
                            (1..=10).into(),                 // RangeInclusive
                            (500..2_000).into(),             // Range
                            (9_999..=100_000_000).into(),    // RangeInclusive
                            [30, 90, 700].as_slice().into()  // Slice
                        ]
                        .as_slice()
                    ),
                    true
                );
                assert_eq!(n.in_range(1_000_000_000..1_000_000_001), false);
            }

            assert_eq!('9'.is_ascii_numeric(), true);
            assert_eq!('T'.is_ascii_numeric(), false);

            assert_eq!('9'.is_ascii_alpha(), false);
            assert_eq!('T'.is_ascii_alpha(), true);

            for c in "9T".chars() {
                assert_eq!(c.is_ascii_alpha_numeric(), true);
            }
        }
    }
}
