pub mod bitbuf {
    static BMAP_U8: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    static BMAP_U16: [u16; 16] = [
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
    ];
    static BMAP_U32: [u32; 32] = [
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
        262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864,
        134217728, 268435456, 536870912, 1073741824, 2147483648,
    ];

    static BMAP_U64: [u64; 64] = [
        1,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
        256,
        512,
        1024,
        2048,
        4096,
        8192,
        16384,
        32768,
        65536,
        131072,
        262144,
        524288,
        1048576,
        2097152,
        4194304,
        8388608,
        16777216,
        33554432,
        67108864,
        134217728,
        268435456,
        536870912,
        1073741824,
        2147483648,
        4294967296,
        8589934592,
        17179869184,
        34359738368,
        68719476736,
        137438953472,
        274877906944,
        549755813888,
        1099511627776,
        2199023255552,
        4398046511104,
        8796093022208,
        17592186044416,
        35184372088832,
        70368744177664,
        140737488355328,
        281474976710656,
        562949953421312,
        1125899906842624,
        2251799813685248,
        4503599627370496,
        9007199254740992,
        18014398509481984,
        36028797018963968,
        72057594037927936,
        144115188075855872,
        288230376151711744,
        576460752303423488,
        1152921504606846976,
        2305843009213693952,
        4611686018427387904,
        9223372036854775808,
    ];
    static BMAP_U128: [u128; 128] = [
        1,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
        256,
        512,
        1024,
        2048,
        4096,
        8192,
        16384,
        32768,
        65536,
        131072,
        262144,
        524288,
        1048576,
        2097152,
        4194304,
        8388608,
        16777216,
        33554432,
        67108864,
        134217728,
        268435456,
        536870912,
        1073741824,
        2147483648,
        4294967296,
        8589934592,
        17179869184,
        34359738368,
        68719476736,
        137438953472,
        274877906944,
        549755813888,
        1099511627776,
        2199023255552,
        4398046511104,
        8796093022208,
        17592186044416,
        35184372088832,
        70368744177664,
        140737488355328,
        281474976710656,
        562949953421312,
        1125899906842624,
        2251799813685248,
        4503599627370496,
        9007199254740992,
        18014398509481984,
        36028797018963968,
        72057594037927936,
        144115188075855872,
        288230376151711744,
        576460752303423488,
        1152921504606846976,
        2305843009213693952,
        4611686018427387904,
        9223372036854775808,
        18446744073709551616,
        36893488147419103232,
        73786976294838206464,
        147573952589676412928,
        295147905179352825856,
        590295810358705651712,
        1180591620717411303424,
        2361183241434822606848,
        4722366482869645213696,
        9444732965739290427392,
        18889465931478580854784,
        37778931862957161709568,
        75557863725914323419136,
        151115727451828646838272,
        302231454903657293676544,
        604462909807314587353088,
        1208925819614629174706176,
        2417851639229258349412352,
        4835703278458516698824704,
        9671406556917033397649408,
        19342813113834066795298816,
        38685626227668133590597632,
        77371252455336267181195264,
        154742504910672534362390528,
        309485009821345068724781056,
        618970019642690137449562112,
        1237940039285380274899124224,
        2475880078570760549798248448,
        4951760157141521099596496896,
        9903520314283042199192993792,
        19807040628566084398385987584,
        39614081257132168796771975168,
        79228162514264337593543950336,
        158456325028528675187087900672,
        316912650057057350374175801344,
        633825300114114700748351602688,
        1267650600228229401496703205376,
        2535301200456458802993406410752,
        5070602400912917605986812821504,
        10141204801825835211973625643008,
        20282409603651670423947251286016,
        40564819207303340847894502572032,
        81129638414606681695789005144064,
        162259276829213363391578010288128,
        324518553658426726783156020576256,
        649037107316853453566312041152512,
        1298074214633706907132624082305024,
        2596148429267413814265248164610048,
        5192296858534827628530496329220096,
        10384593717069655257060992658440192,
        20769187434139310514121985316880384,
        41538374868278621028243970633760768,
        83076749736557242056487941267521536,
        166153499473114484112975882535043072,
        332306998946228968225951765070086144,
        664613997892457936451903530140172288,
        1329227995784915872903807060280344576,
        2658455991569831745807614120560689152,
        5316911983139663491615228241121378304,
        10633823966279326983230456482242756608,
        21267647932558653966460912964485513216,
        42535295865117307932921825928971026432,
        85070591730234615865843651857942052864,
        170141183460469231731687303715884105728,
    ];

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
    /// # use cj_common::cj_binary::bitbuf::bitbuf::BitFlagIter;
    /// let mut x = 0xABu8;
    ///
    /// let mut i = x.bit_iter();
    /// assert_eq!(i.next(), Some(true));
    /// assert_eq!(i.next(), Some(true));
    /// assert_eq!(i.next(), Some(false));
    /// assert_eq!(i.next(), Some(true));
    /// ```
    /// ___
    /// ```
    /// # use cj_common::cj_binary::bitbuf::bitbuf::BitFlagIter;
    /// let mut x = 0xABu8;
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
        /// # use cj_common::cj_binary::bitbuf::bitbuf::Bitflag;
        /// let x = 0b00000010u8;
        /// assert_eq!(x.get_bit(1),true);
        /// ```
        fn get_bit(&self, bit_pos: usize) -> bool;
        /// sets the bit value at the specified position.
        /// * the call is ignored if the bit_pos is out of range
        /// ```
        /// # use cj_common::cj_binary::bitbuf::bitbuf::Bitflag;
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
                    let v = BMAP_U8[bit_pos];
                    self & v == v
                }
                _ => false,
            }
        }

        fn set_bit(&mut self, bit_pos: usize, value: bool) {
            match bit_pos {
                0..=7 => {
                    let v = BMAP_U8[bit_pos];
                    let i = 0xFF - v;
                    if value {
                        *self = (*self & i) + v;
                    } else {
                        *self = *self & i;
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
                    let v = BMAP_U16[bit_pos];
                    self & v == v
                }
                _ => false,
            }
        }

        fn set_bit(&mut self, bit_pos: usize, value: bool) {
            match bit_pos {
                0..=15 => {
                    let v = BMAP_U16[bit_pos];
                    let i = 0xFFFF - v;
                    if value {
                        *self = (*self & i) + v;
                    } else {
                        *self = *self & i;
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
                    let v = BMAP_U32[bit_pos];
                    self & v == v
                }
                _ => false,
            }
        }

        fn set_bit(&mut self, bit_pos: usize, value: bool) {
            match bit_pos {
                0..=31 => {
                    let v = BMAP_U32[bit_pos];
                    let i = 0xFFFFFFFF - v;
                    if value {
                        *self = (*self & i) + v;
                    } else {
                        *self = *self & i;
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
                    let v = BMAP_U64[bit_pos];
                    self & v == v
                }
                _ => false,
            }
        }

        fn set_bit(&mut self, bit_pos: usize, value: bool) {
            match bit_pos {
                0..=63 => {
                    let v = BMAP_U64[bit_pos];
                    let i = 0xFFFFFFFFFFFFFFFF - v;
                    if value {
                        *self = (*self & i) + v;
                    } else {
                        *self = *self & i;
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
                    let v = BMAP_U128[bit_pos];
                    self & v == v
                }
                _ => false,
            }
        }

        fn set_bit(&mut self, bit_pos: usize, value: bool) {
            match bit_pos {
                0..=127 => {
                    let v = BMAP_U128[bit_pos];
                    let i = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF - v;
                    if value {
                        *self = (*self & i) + v;
                    } else {
                        *self = *self & i;
                    }
                }
                _ => {}
            }
        }
    }

    #[cfg(test)]
    mod test {
        use crate::cj_binary::bitbuf::bitbuf::{BitFlagIter, Bitflag};

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
            let mut x = 0xABu8;

            let mut i = x.bit_iter();
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(false));
            assert_eq!(i.next(), Some(true));
        }

        #[test]
        fn test_u16_iter() {
            let mut x = 0xABu16;

            let mut i = x.bit_iter();
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(false));
            assert_eq!(i.next(), Some(true));
        }

        #[test]
        fn test_u32_iter() {
            let mut x = 0xABu32;

            let mut i = x.bit_iter();
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(false));
            assert_eq!(i.next(), Some(true));
        }

        #[test]
        fn test_u64_iter() {
            let mut x = 0xABu64;

            let mut i = x.bit_iter();
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(false));
            assert_eq!(i.next(), Some(true));
        }

        #[test]
        fn test_u128_iter() {
            let mut x = 0xABu128;

            let mut i = x.bit_iter();
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(true));
            assert_eq!(i.next(), Some(false));
            assert_eq!(i.next(), Some(true));
        }

        #[test]
        fn test_u8_iter2() {
            let mut x = 0xABu8;
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
}
