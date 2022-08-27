pub mod in_set {

    use std::ops::{Range, RangeInclusive};

    trait CjExactRng<T: PartialOrd + PartialEq> {
        fn in_exact_range(&self, value: &T) -> bool;
    }

    impl<T: PartialOrd + PartialEq> CjExactRng<T> for Range<T> {
        fn in_exact_range(&self, value: &T) -> bool {
            self.contains(value)
        }
    }

    impl<T: PartialOrd + PartialEq> CjExactRng<T> for RangeInclusive<T> {
        fn in_exact_range(&self, value: &T) -> bool {
            self.contains(value)
        }
    }

    impl<'a, T: PartialOrd + PartialEq> CjExactRng<T> for CjExactRange<'a, T> {
        fn in_exact_range(&self, value: &T) -> bool {
            match &self.inner {
                RangeType::Exact(s, e) => (s <= value) && (e >= value),
                RangeType::Slice(s) => s.contains(value),
                //RangeType::Array(s) => s.contains(value),
            }
        }
    }

    impl<'a, T> Into<CjExactRange<'a, T>> for Range<T> {
        fn into(self) -> CjExactRange<'a, T> {
            CjExactRange {
                inner: RangeType::Exact(self.start, self.end),
            }
        }
    }

    impl<'a, T> Into<CjExactRange<'a, T>> for RangeInclusive<T> {
        fn into(self) -> CjExactRange<'a, T> {
            let (s, e) = self.into_inner();
            CjExactRange {
                inner: RangeType::Exact(s, e),
            }
        }
    }

    impl<'a, T> Into<CjExactRange<'a, T>> for &'a [T] {
        fn into(self) -> CjExactRange<'a, T> {
            CjExactRange {
                inner: RangeType::Slice(self),
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum RangeType<'a, T> {
        Exact(T, T),
        Slice(&'a [T]),
        //Array(&'a [T;N]),
    }

    /// CjExactRange is similar to RangeInclusive and is used by the in_set() method.
    /// in_set() requires CjExactRange in order to support a mixed slice of Range, RangeInclusive and Slice.
    /// Note that Range<T>.into(), RangeInclusive<T>.into() and Slice<T>.into() have been implemented
    /// for CjExactRange for easy conversion.
    /// ```
    /// # use cj_common::prelude::CjInSets;
    /// assert_eq!(
    ///    'z'.in_set(
    ///         [
    ///             ('a'..'r').into(),               // Range
    ///             ('r'..='z').into(),              // RangeInclusive
    ///             ['a','b','c'].as_slice().into(), // Slice
    ///         ].as_slice()
    ///     ),
    ///     true
    /// );
    /// ```
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct CjExactRange<'a, T> {
        inner: RangeType<'a, T>,
    }

    trait CjInExactSet<T: PartialEq + PartialOrd> {
        fn in_exact_set(&self, value: &T) -> bool;
    }

    impl<'a, T: PartialEq + PartialOrd> CjInExactSet<T> for CjExactRange<'a, T> {
        fn in_exact_set(&self, value: &T) -> bool {
            self.in_exact_range(value)
        }
    }

    impl<'a, T: PartialEq + PartialOrd> CjInExactSet<T> for &[CjExactRange<'a, T>] {
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
        fn in_range_inclusive(&self, value: RangeInclusive<T>) -> bool {
            value.contains(self)
        }
    }

    pub trait CjInSets<T: PartialEq + PartialOrd> {
        /// Returns true if a value is within a give slice of ranges.
        /// Note that this method requires ranges to be of type CjExactRange,
        /// so Range<T>.into(), RangeInclusive<T>.into() and Slice<T>.into() have been implemented
        /// for CjExactRange for easy conversion.
        /// ```
        /// # use cj_common::prelude::CjInSets;
        /// assert_eq!(
        ///    'z'.in_set(
        ///         [
        ///             ('a'..'r').into(),  // Range
        ///             ('r'..='z').into(),  // RangeInclusive
        ///             ['a','b','c'].as_slice().into(), // Slice
        ///         ].as_slice()
        ///     ),
        ///     true
        /// );
        /// ```
        fn in_set(&self, value: &[CjExactRange<T>]) -> bool;
    }

    impl<T: PartialEq + PartialOrd> CjInSets<T> for T {
        fn in_set(&self, value: &[CjExactRange<T>]) -> bool {
            value.in_exact_set(self)
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
                            (9_999..=100_000_000).into()
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
                            ['a', 'b', 'c'].as_slice().into()  // Slice
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
        }
    }
}
