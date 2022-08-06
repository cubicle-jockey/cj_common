pub mod b64 {
    // general b64 table (RFC 4648.4).
    static B64_TABLE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    // fn bytes_to_b64(bytes: &[u8]) -> String {
    //     let mut s = String::with_capacity((bytes.len() as f64 * 1.25) as usize + 2);
    //
    //
    //     let mut three_hit = false;
    //     let mut x = 0i32;
    //     let mut tmp = 0u32;
    //     for b in bytes[..].iter() {
    //         tmp += *b as u32;
    //         tmp = tmp << 8;
    //         x += 1;
    //         if x == 3 {
    //             tmp = tmp >> 2;
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //             tmp = tmp << 8;
    //             tmp = tmp >> 2;
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //             tmp = tmp << 8;
    //             tmp = tmp >> 2;
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //
    //             tmp = tmp << 2;
    //
    //             tmp = tmp >> 20;
    //             tmp = tmp << 8;
    //             x = 0;
    //             three_hit = true;
    //         }
    //     }
    //     // broken from here down vvvv
    //     match x {
    //         1 => {
    //             // 0000 0000  0000 0000  0000 0000  0000 0000
    //
    //             //println!("one hit");
    //             //println!("{:x?}", &tmp.to_be_bytes());
    //             let b2 = &tmp.to_be_bytes()[1];
    //             s.push(B64_TABLE[*b2 as usize]);
    //             tmp = tmp << 16;
    //             tmp = tmp >> 2;
    //             //println!("{:x?}", &tmp.to_be_bytes());
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //
    //             tmp = tmp << 8;
    //             tmp = tmp >> 2;
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //
    //             s.push_str("==");
    //         }
    //         2 => {
    //             //println!("two hit");
    //             //println!("{:x?}", &tmp.to_be_bytes());
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //             tmp = tmp << 8;
    //             tmp = tmp >> 2;
    //             //println!("{:x?}", &tmp.to_be_bytes());
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //
    //             tmp = tmp << 8;
    //             tmp = tmp >> 2;
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //
    //             tmp = tmp << 8;
    //             tmp = tmp >> 2;
    //             let b2 = &tmp.to_be_bytes()[0];
    //             s.push(B64_TABLE[*b2 as usize]);
    //
    //             s.push_str("=");
    //         }
    //         _ => {
    //             if three_hit {
    //                 let b2 = &tmp.to_be_bytes()[2];
    //                 s.push(B64_TABLE[*b2 as usize]);
    //             }
    //         }
    //     }
    //
    //     s
    // }
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
        match pass_of {
            0 => {
                // take 6 from src
                *b6 = src >> 2;
                // two remaining
                *br = src << 6;
                *pass_of += 1;

                return BitSplit6Result::Ready;
            }
            1 => {
                // take two remaining
                *b6 = *br >> 2;
                // plus 4 from src
                *b6 += src >> 4;
                // four remaining
                *br = src << 4;
                *pass_of += 1;

                return BitSplit6Result::Ready;
            }
            2 => {
                // take four remaining
                *b6 = *br >> 2;
                // plus two from src
                *b6 += src >> 6;
                // six remaining
                *br = src << 2;
                *pass_of += 1;

                return BitSplit6Result::Ready;
            }
            3 => {
                // take 6 remaining
                *b6 = *br >> 2;
                *pass_of = 0;

                return BitSplit6Result::Resend;
            }
            _ => {
                *pass_of = 0;
                //println!("huh? not expected here");
                return BitSplit6Result::Resend;
            }
        }
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

    #[cfg(test)]
    mod test {
        use crate::cj_binary::b64::b64::bytes_to_b64;

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
    }
}
