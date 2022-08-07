pub mod b64 {
    use std::slice::Iter;
    use std::str::Chars;

    use crate::prelude::CharToOrdResult::{Invalid, Pad, WhiteSpace};

    // general b64 table (RFC 4648.4).
    static B64_TABLE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    #[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
    pub enum CharToOrdResult {
        Ok(u8),
        Pad,
        WhiteSpace,
        Invalid,
    }

    impl CharToOrdResult {
        #[inline]
        pub fn is_ok(&self) -> bool {
            *self < Pad
        }
        #[inline]
        pub fn is_padding(&self) -> bool {
            *self == Pad
        }
        #[inline]
        pub fn is_whitespace(&self) -> bool {
            *self == WhiteSpace
        }
        #[inline]
        pub fn is_invalid(&self) -> bool {
            *self == Invalid
        }
    }

    #[inline]
    pub fn b64_char_to_ord(c: &char) -> &'static CharToOrdResult {
        match c {
            'A' => &CharToOrdResult::Ok(0),
            'B' => &CharToOrdResult::Ok(1),
            'C' => &CharToOrdResult::Ok(2),
            'D' => &CharToOrdResult::Ok(3),
            'E' => &CharToOrdResult::Ok(4),
            'F' => &CharToOrdResult::Ok(5),
            'G' => &CharToOrdResult::Ok(6),
            'H' => &CharToOrdResult::Ok(7),
            'I' => &CharToOrdResult::Ok(8),
            'J' => &CharToOrdResult::Ok(9),
            'K' => &CharToOrdResult::Ok(10),
            'L' => &CharToOrdResult::Ok(11),
            'M' => &CharToOrdResult::Ok(12),
            'N' => &CharToOrdResult::Ok(13),
            'O' => &CharToOrdResult::Ok(14),
            'P' => &CharToOrdResult::Ok(15),
            'Q' => &CharToOrdResult::Ok(16),
            'R' => &CharToOrdResult::Ok(17),
            'S' => &CharToOrdResult::Ok(18),
            'T' => &CharToOrdResult::Ok(19),
            'U' => &CharToOrdResult::Ok(20),
            'V' => &CharToOrdResult::Ok(21),
            'W' => &CharToOrdResult::Ok(22),
            'X' => &CharToOrdResult::Ok(23),
            'Y' => &CharToOrdResult::Ok(24),
            'Z' => &CharToOrdResult::Ok(25),
            'a' => &CharToOrdResult::Ok(26),
            'b' => &CharToOrdResult::Ok(27),
            'c' => &CharToOrdResult::Ok(28),
            'd' => &CharToOrdResult::Ok(29),
            'e' => &CharToOrdResult::Ok(30),
            'f' => &CharToOrdResult::Ok(31),
            'g' => &CharToOrdResult::Ok(32),
            'h' => &CharToOrdResult::Ok(33),
            'i' => &CharToOrdResult::Ok(34),
            'j' => &CharToOrdResult::Ok(35),
            'k' => &CharToOrdResult::Ok(36),
            'l' => &CharToOrdResult::Ok(37),
            'm' => &CharToOrdResult::Ok(38),
            'n' => &CharToOrdResult::Ok(39),
            'o' => &CharToOrdResult::Ok(40),
            'p' => &CharToOrdResult::Ok(41),
            'q' => &CharToOrdResult::Ok(42),
            'r' => &CharToOrdResult::Ok(43),
            's' => &CharToOrdResult::Ok(44),
            't' => &CharToOrdResult::Ok(45),
            'u' => &CharToOrdResult::Ok(46),
            'v' => &CharToOrdResult::Ok(47),
            'w' => &CharToOrdResult::Ok(48),
            'x' => &CharToOrdResult::Ok(49),
            'y' => &CharToOrdResult::Ok(50),
            'z' => &CharToOrdResult::Ok(51),
            '0' => &CharToOrdResult::Ok(52),
            '1' => &CharToOrdResult::Ok(53),
            '2' => &CharToOrdResult::Ok(54),
            '3' => &CharToOrdResult::Ok(55),
            '4' => &CharToOrdResult::Ok(56),
            '5' => &CharToOrdResult::Ok(57),
            '6' => &CharToOrdResult::Ok(58),
            '7' => &CharToOrdResult::Ok(59),
            '8' => &CharToOrdResult::Ok(60),
            '9' => &CharToOrdResult::Ok(61),
            '+' => &CharToOrdResult::Ok(62),
            '/' => &CharToOrdResult::Ok(63),
            '=' => &Pad,
            // chars to ignore
            ' ' => &WhiteSpace,
            '\r' => &WhiteSpace,
            '\n' => &WhiteSpace,
            '\t' => &WhiteSpace,
            // all else are failures
            _ => &Invalid,
        }
    }

    enum BitSplit6Result {
        Ready,
        Resend,
    }

    #[inline(always)]
    fn bit_split_6(src: &u8, b6: &mut u8, br: &mut u8, pass_of: &mut usize) -> BitSplit6Result {
        // iter  bytes (it requires 4 bytes, per 3 bytes in)
        // -     11111111 22222222 33333333 00000000 (start)
        // 0     XX111111 11222222 22333333 33000000
        // 1     XXXX1111 11112222 22223333 33330000
        // 2     XXXXXX11 11111122 22222233 33333300
        // 3     XXXXXXXX 11111111 22222222 33333333
        return match pass_of {
            0 => {
                // take 6 from src
                *b6 = src >> 2;
                // two remaining
                *br = src << 6;
                *pass_of += 1;

                BitSplit6Result::Ready
            }
            1 => {
                // take two remaining
                *b6 = *br >> 2;
                // plus 4 from src
                *b6 += src >> 4;
                // four remaining
                *br = src << 4;
                *pass_of += 1;

                BitSplit6Result::Ready
            }
            2 => {
                // take four remaining
                *b6 = *br >> 2;
                // plus two from src
                *b6 += src >> 6;
                // six remaining
                *br = src << 2;
                *pass_of += 1;

                BitSplit6Result::Ready
            }
            3 => {
                // take 6 remaining
                *b6 = *br >> 2;
                *pass_of = 0;

                BitSplit6Result::Resend
            }
            _ => {
                *pass_of = 0;
                //println!("huh? not expected here");
                BitSplit6Result::Resend
            }
        };
    }

    ///
    /// converts a slice of bytes into a base64 encoded string
    ///
    /// ```
    ///  # use crate::cj_common::cj_binary::b64::b64::bytes_to_b64;
    ///
    /// let s = "Many hands make light work.".as_bytes();
    /// let s2 = bytes_to_b64(s);
    /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
    /// ```
    pub fn bytes_to_b64(bytes: &[u8]) -> String {
        let mut s = String::with_capacity((bytes.len() as f64 * 1.25) as usize + 2);

        let mut b_six = 0u8;
        let mut b_rem = 0u8;
        let mut pass_no = 0usize;
        let b_len = bytes.len();
        let mut s_char_ct = 0;

        for b in bytes[..].iter() {
            match bit_split_6(b, &mut b_six, &mut b_rem, &mut pass_no) {
                BitSplit6Result::Ready => {
                    s.push(B64_TABLE[b_six as usize]);
                    s_char_ct += 1;
                }
                BitSplit6Result::Resend => {
                    s.push(B64_TABLE[b_six as usize]);
                    let _ = bit_split_6(b, &mut b_six, &mut b_rem, &mut pass_no);
                    s.push(B64_TABLE[b_six as usize]);
                    s_char_ct += 2;
                }
            }
        }

        if b_len > 0 {
            let x = b_rem.clone();
            b_rem = 0;
            pass_no = 0;
            let _ = bit_split_6(&x, &mut b_six, &mut b_rem, &mut pass_no);
            s.push(B64_TABLE[b_six as usize]);
            s_char_ct += 1;
        }

        while ((s_char_ct * 6) % 8) != 0 {
            s.push('=');
            s_char_ct += 1;
        }
        s
    }

    ///
    /// Iterator for a slice of bytes that returns Base64
    ///
    /// ```
    ///  # use cj_common::prelude::CjToBase64Iter;
    ///  # use crate::cj_common::cj_binary::b64::b64::ToBase64Iter;
    /// // slice of bytes example
    /// let s = "Many hands make light work.".as_bytes();
    /// let mut s2 = String::new();
    /// for c in s.iter_base64() {
    ///     s2.push(c);
    /// }
    /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
    ///
    /// // vec of bytes example
    /// let s = "Many hands make light work.".as_bytes().to_vec();
    /// let mut s2 = String::new();
    ///
    /// for c in s.iter_base64() {
    ///     s2.push(c);
    /// }
    /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
    /// ```
    pub struct ToBase64Iter<'a> {
        six_bits: u8,
        rem_bits: u8,
        pass_no: usize,
        char_count: usize,
        push_last: bool,
        pend_char: Option<char>,
        inner: Iter<'a, u8>,
    }

    impl<'a> ToBase64Iter<'a> {
        pub fn new(i: Iter<'a, u8>) -> Self {
            Self {
                six_bits: 0,
                rem_bits: 0,
                pass_no: 0,
                pend_char: None,
                char_count: 0,
                push_last: true,
                inner: i,
            }
        }

        fn next_char(&mut self) -> Option<char> {
            if let Some(c) = self.pend_char {
                self.pend_char = None;
                return Some(c);
            } else {
                if let Some(b) = &self.inner.next() {
                    let c: char;
                    match bit_split_6(b, &mut self.six_bits, &mut self.rem_bits, &mut self.pass_no)
                    {
                        BitSplit6Result::Ready => {
                            c = B64_TABLE[self.six_bits as usize];
                            self.char_count += 1;
                        }
                        BitSplit6Result::Resend => {
                            c = B64_TABLE[self.six_bits as usize];
                            let _ = bit_split_6(
                                b,
                                &mut self.six_bits,
                                &mut self.rem_bits,
                                &mut self.pass_no,
                            );
                            self.pend_char = Some(B64_TABLE[self.six_bits as usize]);
                            self.char_count += 2;
                        }
                    }
                    return Some(c);
                } else if self.char_count > 0 {
                    if self.push_last {
                        let x = self.rem_bits.clone();
                        self.rem_bits = 0;
                        self.pass_no = 0;
                        let _ = bit_split_6(
                            &x,
                            &mut self.six_bits,
                            &mut self.rem_bits,
                            &mut self.pass_no,
                        );
                        self.char_count += 1;
                        self.push_last = false;
                        return Some(B64_TABLE[self.six_bits as usize]);
                    } else if ((self.char_count * 6) % 8) != 0 {
                        self.char_count += 1;
                        return Some('=');
                    }
                }
                None
            }
        }
    }

    impl Iterator for ToBase64Iter<'_> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            self.next_char()
        }
    }

    pub trait CjToBase64Iter {
        fn iter_base64(&self) -> ToBase64Iter;
    }

    impl CjToBase64Iter for &[u8] {
        /// Iterator for a slice of bytes that produces Base64 encoded chars
        /// ```
        /// # use cj_common::prelude::CjToBase64Iter;
        /// let s = "Many hands make light work.".as_bytes();
        /// let mut s2 = String::new();
        /// for c in s.iter_base64() {
        ///     s2.push(c);
        /// }
        /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        /// ```
        fn iter_base64(&self) -> ToBase64Iter {
            ToBase64Iter::new(self[..].iter())
        }
    }

    impl CjToBase64Iter for &str {
        /// Iterator for str that produces Base64 encoded chars
        /// ```
        /// # use cj_common::prelude::CjToBase64Iter;
        /// let mut s2 = String::new();
        /// for c in "Many hands make light work.".iter_base64() {
        ///     s2.push(c);
        /// }
        /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        /// ```
        fn iter_base64(&self) -> ToBase64Iter {
            ToBase64Iter::new(self.as_bytes()[..].iter())
        }
    }

    impl CjToBase64Iter for Vec<u8> {
        /// Iterator for a Vec<u8> that produces Base64 encoded chars
        /// ```
        /// # use cj_common::prelude::CjToBase64Iter;
        /// let s = Vec::<u8>::from("Many hands make light work.");
        /// let mut s2 = String::new();
        /// for c in s.iter_base64() {
        ///     s2.push(c);
        /// }
        /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        /// ```
        fn iter_base64(&self) -> ToBase64Iter {
            ToBase64Iter::new(self[..].iter())
        }
    }

    ///
    /// Iterator for a Base64 encoded str that returns decoded byte
    ///
    /// ```
    ///  # use cj_common::prelude::CjFromBase64Iter;
    ///  # use crate::cj_common::cj_binary::b64::b64::FromBase64Iter;
    /// // slice of bytes example
    /// let mut v = Vec::new();
    /// for b in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
    ///     v.push(b);
    /// }
    /// assert!(v.len() > 0);
    /// let r = String::from_utf8_lossy(v.as_slice());
    /// let s = "Many hands make light work.";
    /// assert_eq!(r.to_string().as_str(), s);
    /// ```
    pub struct FromBase64Iter<'a> {
        buf_left: u8,
        buf_ct: u8,
        had_pad: bool,
        total_added: usize,
        inner: Chars<'a>,
    }

    impl<'a> FromBase64Iter<'a> {
        pub fn new(i: Chars<'a>) -> Self {
            Self {
                buf_left: 0,
                buf_ct: 0,
                had_pad: false,
                total_added: 0,
                inner: i,
            }
        }

        // fn new_adapter(i: Chars) -> Self {
        //     Self {
        //         buf_left: 0,
        //         buf_ct: 0,
        //         had_pad: false,
        //         total_added: 0,
        //         inner: i,
        //     }
        // }

        fn next_byte(&mut self) -> Option<u8> {
            let return_b: u8;
            while let Some(c) = &self.inner.next() {
                let r = b64_char_to_ord(c);
                match r {
                    CharToOrdResult::Ok(b) => {
                        if self.had_pad {
                            break;
                        }
                        match self.buf_ct {
                            0 => {
                                // buf_left is empty
                                // drop two bits from 'b' and take 6
                                // 00XXXXXX
                                self.buf_left = b << 2;
                                self.buf_ct += 1;
                            }
                            1 => {
                                // buf_left has 6 bits XXXXXX00
                                // drop two bits from 'b' and take 2
                                self.buf_left += (b << 2) >> 6;
                                // push the complete byte
                                return_b = self.buf_left.clone();
                                // now take the remaining 4
                                self.buf_left = b << 4;
                                self.buf_ct += 1;
                                self.total_added += 1;
                                return Some(return_b);
                            }
                            2 => {
                                // buf_left has 4 bits XXXX0000
                                // drop two bits from 'b' and take 4
                                self.buf_left += (b << 2) >> 4;
                                // push the complete byte
                                return_b = self.buf_left.clone();
                                self.buf_left = b << 6;
                                self.buf_ct += 1;
                                self.total_added += 1;
                                return Some(return_b);
                            }
                            3 => {
                                // buf_left has 2 bits XX000000
                                // 'b' only has 6 bits. take them all
                                self.buf_left += b;
                                // push the complete byte
                                return_b = self.buf_left.clone();
                                self.buf_ct = 0;
                                self.total_added += 1;
                                return Some(return_b);
                            }
                            _ => unreachable!(),
                        }
                    }
                    Pad => self.had_pad = true,
                    WhiteSpace => {
                        if self.had_pad {
                            break;
                        }
                    }
                    Invalid => {
                        return None;
                    }
                }
            }

            if self.total_added > 0 {
                if (self.buf_ct != 0) && (!self.had_pad) {
                    return Some(self.buf_left.clone());
                }
            }

            None
        }
    }

    impl Iterator for FromBase64Iter<'_> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            self.next_byte()
        }
    }

    pub trait CjFromBase64Iter {
        fn iter_b64_to_byte(&self) -> FromBase64Iter;
    }

    impl CjFromBase64Iter for &str {
        /// Iterator for decoding a Base64 endoced str to bytes
        /// ```
        /// # use cj_common::prelude::CjFromBase64Iter;
        /// let mut v = Vec::new();
        /// for b in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
        ///     v.push(b);
        /// }
        /// assert!(v.len() > 0);
        /// let r = String::from_utf8_lossy(v.as_slice());
        /// let s = "Many hands make light work.";
        /// assert_eq!(r.to_string().as_str(), s);
        /// ```
        fn iter_b64_to_byte(&self) -> FromBase64Iter {
            FromBase64Iter::new(self.chars())
        }
    }

    /// converts a Base64 encoded str into a vec of bytes
    /// ```
    /// # use cj_common::prelude::base64_to_bytes;
    /// let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
    /// assert!(v.is_some());
    /// if let Some(x) = v {
    ///    let s = "Many hands make light work.".as_bytes().to_vec();
    ///    assert_eq!(x, s);
    /// }
    /// ```
    pub fn base64_to_bytes(data: &str) -> Option<Vec<u8>> {
        let mut v = Vec::<u8>::with_capacity((data.len() as f64 * 0.8) as usize);
        let mut buf_left = 0u8;
        let mut buf_ct = 0usize;
        let mut had_pad = false;
        //let mut had_error = false;
        let mut total_added = 0usize;
        for c in data.chars() {
            let r = b64_char_to_ord(&c);
            match r {
                CharToOrdResult::Ok(b) => {
                    if had_pad {
                        break;
                    }
                    match buf_ct {
                        0 => {
                            // buf_left is empty
                            // drop two bits from 'b' and take 6
                            // 00XXXXXX
                            buf_left = b << 2;
                            buf_ct += 1;
                        }
                        1 => {
                            // buf_left has 6 bits XXXXXX00
                            // drop two bits from 'b' and take 2
                            buf_left += (b << 2) >> 6;
                            // push the complete byte
                            v.push(buf_left);
                            // now take the remaining 4
                            buf_left = b << 4;
                            buf_ct += 1;
                            total_added += 1;
                        }
                        2 => {
                            // buf_left has 4 bits XXXX0000
                            // drop two bits from 'b' and take 4
                            buf_left += (b << 2) >> 4;
                            // push the complete byte
                            v.push(buf_left);
                            buf_left = b << 6;
                            buf_ct += 1;
                            total_added += 1;
                        }
                        3 => {
                            // buf_left has 2 bits XX000000
                            // 'b' only has 6 bits. take them all
                            buf_left += b;
                            // push the complete byte
                            v.push(buf_left);
                            buf_ct = 0;
                            total_added += 1;
                        }
                        _ => unreachable!(),
                    }
                }
                Pad => had_pad = true,
                WhiteSpace => {
                    if had_pad {
                        break;
                    }
                }
                Invalid => {
                    //had_error = true;
                    break;
                }
            }
        }

        if total_added > 0 {
            if (buf_ct != 0) && (!had_pad) {
                v.push(buf_left);
            }
            // we were able to decode something, even if there was an error. return what we have.
            return Some(v);
        }

        None
    }

    pub trait CjToBase64 {
        fn to_base64_string(&self) -> String;
    }

    impl CjToBase64 for &str {
        /// converts str to Base64 encoded String
        /// ```
        /// # use cj_common::prelude::CjToBase64;
        /// let s = "Many hands make light work...8675";
        /// let s2 = s.to_base64_string();
        /// println!("{}", s2);
        /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        /// ```
        fn to_base64_string(&self) -> String {
            bytes_to_b64(self.as_bytes())
        }
    }

    impl CjToBase64 for String {
        /// converts String to Base64 encoded String
        /// ```
        /// # use cj_common::prelude::CjToBase64;
        /// let s = String::from("Many hands make light work...8675");
        /// let s2 = s.to_base64_string();
        /// println!("{}", s2);
        /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        /// ```
        fn to_base64_string(&self) -> String {
            bytes_to_b64(self.as_bytes())
        }
    }

    impl CjToBase64 for Vec<u8> {
        /// converts Vec<u8> to Base64 encoded String
        /// ```
        /// # use cj_common::prelude::CjToBase64;
        /// let s = Vec::<u8>::from("Many hands make light work...8675");
        /// let s2 = s.to_base64_string();
        /// println!("{}", s2);
        /// assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        /// ```
        fn to_base64_string(&self) -> String {
            bytes_to_b64(self.as_slice())
        }
    }

    #[cfg(test)]
    mod test {
        use crate::cj_binary::b64::b64::*;

        #[test]
        fn test_1() {
            let s = "Many hands make light work.".as_bytes();
            let s2 = bytes_to_b64(s);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        }

        #[test]
        fn test_2() {
            let s = "Many hands make light work..".as_bytes();
            let s2 = bytes_to_b64(s);
            println!("{}", s2);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLg==");
        }

        #[test]
        fn test_3() {
            let s = "Many hands make light work...".as_bytes();
            let s2 = bytes_to_b64(s);
            println!("{}", s2);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi4=");
        }

        #[test]
        fn test_4() {
            let s = "Many hands make light work...8675".as_bytes();
            let s2 = bytes_to_b64(s);
            println!("{}", s2);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        }

        #[test]
        fn test_5() {
            let s = "Many hands make light work...8675";
            let s2 = s.to_base64_string();
            println!("{}", s2);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        }

        #[test]
        fn test_6() {
            let s = String::from("Many hands make light work...8675");
            let s2 = s.to_base64_string();
            println!("{}", s2);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        }

        #[test]
        fn test_7() {
            let s = Vec::<u8>::from("Many hands make light work...8675");
            let s2 = s.to_base64_string();
            println!("{}", s2);
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi44Njc1");
        }

        #[test]
        fn test_iter_1() {
            let s = "Many hands make light work.".as_bytes();
            let mut s2 = String::new();

            for c in s.iter_base64() {
                s2.push(c);
            }
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        }

        #[test]
        fn test_iter_2() {
            let s = "Many hands make light work.".as_bytes().to_vec();
            let mut s2 = String::new();

            for c in s.iter_base64() {
                s2.push(c);
            }
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        }

        #[test]
        fn test_iter_3() {
            let mut s2 = String::new();
            for c in "Many hands make light work.".iter_base64() {
                s2.push(c);
            }
            assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        }

        #[test]
        fn test_from_b64_1() {
            let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
            assert!(v.is_some());
            if let Some(x) = v {
                let s = "Many hands make light work.".as_bytes().to_vec();
                assert_eq!(x, s);
            }
        }

        #[test]
        fn test_from_b64_2() {
            let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLg==");
            assert!(v.is_some());
            if let Some(x) = v {
                let s = "Many hands make light work..".as_bytes().to_vec();
                assert_eq!(x, s);
            }
        }

        #[test]
        fn test_from_b64_3() {
            let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsuLi4=");
            assert!(v.is_some());
            if let Some(x) = v {
                let s = "Many hands make light work...".as_bytes().to_vec();
                assert_eq!(x, s);
            }
        }

        #[test]
        fn test_from_b64_4() {
            let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtl   IGxpZ2h0IHdvcmsuLi4=");
            assert!(v.is_some());
            if let Some(x) = v {
                let s = "Many hands make light work...".as_bytes().to_vec();
                assert_eq!(x, s);
            }
        }

        #[test]
        fn test_from_b64_5() {
            let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtl   IGxpZ2h0IHdvcmsuLi4=nope");
            assert!(v.is_some());
            if let Some(x) = v {
                let s = "Many hands make light work...".as_bytes().to_vec();
                assert_eq!(x, s);
            }
        }

        #[test]
        fn test_from_b64_6() {
            let v = base64_to_bytes("TWFueSBoYW5kcyBtYWtl   IGxpZ2h0IHdvcmsuLi4=&");
            assert!(v.is_some());
            if let Some(x) = v {
                let s = "Many hands make light work...".as_bytes().to_vec();
                assert_eq!(x, s);
            }
        }

        #[test]
        fn test_from_b64_iter_1() {
            let mut v = Vec::new();
            for b in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
                v.push(b);
            }
            assert!(v.len() > 0);
            let r = String::from_utf8_lossy(v.as_slice());
            let s = "Many hands make light work.";
            assert_eq!(r.to_string().as_str(), s);
        }

        #[test]
        fn test_from_b64_iter_2() {
            let mut v = Vec::new();
            for b in "TWFueSBoYW5kcyBtYWtl   IGxpZ2h0IHdvcmsuLi4=&".iter_b64_to_byte() {
                v.push(b);
            }
            assert!(v.len() > 0);
            let r = String::from_utf8_lossy(v.as_slice());
            let s = "Many hands make light work...";
            assert_eq!(r.to_string().as_str(), s);
        }
    }
}
