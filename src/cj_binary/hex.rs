pub mod hex {
    static HEX_TABLE: [&'static str; 256] = [
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0A", "0B", "0C", "0D", "0E",
        "0F", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1A", "1B", "1C", "1D",
        "1E", "1F", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2A", "2B", "2C",
        "2D", "2E", "2F", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3A", "3B",
        "3C", "3D", "3E", "3F", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4A",
        "4B", "4C", "4D", "4E", "4F", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "5A", "5B", "5C", "5D", "5E", "5F", "60", "61", "62", "63", "64", "65", "66", "67", "68",
        "69", "6A", "6B", "6C", "6D", "6E", "6F", "70", "71", "72", "73", "74", "75", "76", "77",
        "78", "79", "7A", "7B", "7C", "7D", "7E", "7F", "80", "81", "82", "83", "84", "85", "86",
        "87", "88", "89", "8A", "8B", "8C", "8D", "8E", "8F", "90", "91", "92", "93", "94", "95",
        "96", "97", "98", "99", "9A", "9B", "9C", "9D", "9E", "9F", "A0", "A1", "A2", "A3", "A4",
        "A5", "A6", "A7", "A8", "A9", "AA", "AB", "AC", "AD", "AE", "AF", "B0", "B1", "B2", "B3",
        "B4", "B5", "B6", "B7", "B8", "B9", "BA", "BB", "BC", "BD", "BE", "BF", "C0", "C1", "C2",
        "C3", "C4", "C5", "C6", "C7", "C8", "C9", "CA", "CB", "CC", "CD", "CE", "CF", "D0", "D1",
        "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "DA", "DB", "DC", "DD", "DE", "DF", "E0",
        "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "EA", "EB", "EC", "ED", "EE", "EF",
        "F0", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "FA", "FB", "FC", "FD", "FE",
        "FF",
    ];

    /// ```
    /// // converts u8 to hex &str
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u8_to_hex_str(&0xD1), "D1");
    /// ```
    #[inline]
    pub fn u8_to_hex_str(value: &u8) -> &'static str {
        HEX_TABLE[*value as usize]
    }

    /// ```
    /// // converts u8 to hex String
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u8_to_hex(&0xD1), "D1".to_string());
    /// ```
    #[inline]
    pub fn u8_to_hex(value: &u8) -> String {
        HEX_TABLE[*value as usize].to_string()
    }

    /// ```
    /// // converts a u8 slice to hex String
    ///
    /// use cj_common::prelude::*;
    ///
    /// let array = [0xA0,0xA1,0xA2];
    /// assert_eq!(u8_array_to_hex(&array),"A0A1A2");
    /// ```
    #[inline]
    pub fn u8_array_to_hex(value: &[u8]) -> String {
        let mut rslt = String::with_capacity(value.len() * 2);
        let _: () = value
            .iter()
            .map(|f| rslt.push_str(u8_to_hex_str(f)))
            .collect();
        rslt
    }

    /// ```
    /// // converts a hex char to u8
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(hex_char_to_u8(&'A'),Some(0x0A));
    /// assert_eq!(hex_char_to_u8(&'G'),None);
    /// ```
    #[inline]
    pub fn hex_char_to_u8(hex1: &char) -> Option<u8> {
        let r = match hex1 {
            '0' => 0u8,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' | 'A' => 10,
            'b' | 'B' => 11,
            'c' | 'C' => 12,
            'd' | 'D' => 13,
            'e' | 'E' => 14,
            'f' | 'F' => 15,
            _ => 255,
        };
        match r {
            255 => None,
            _ => Some(r),
        }
    }

    /// ```
    /// // converts a hex str to u8
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(hex_str_to_u8("AB"),Some(0xAB));
    ///  assert_eq!(hex_str_to_u8("G"),None);
    /// ```
    #[inline]
    pub fn hex_str_to_u8(hex2: &str) -> Option<u8> {
        if hex2.chars().count() > 0 {
            let mut r: u8;
            if let Some(x) = hex_char_to_u8(&hex2.chars().nth(0).unwrap()) {
                r = x << 4;
                if let Some(y) = hex_char_to_u8(&hex2.chars().nth(1).unwrap()) {
                    r += y;
                    Some(r)
                } else {
                    Some(x)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub type HexArray = [char; 2];

    /// ```
    /// // converts a two char hex array to u8
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(hex_chars_to_u8(&['A','B']),Some(0xAB));
    /// assert_eq!(hex_chars_to_u8(&['N','O']),None);
    /// ```
    #[inline]
    pub fn hex_chars_to_u8(hex2: &HexArray) -> Option<u8> {
        let mut r: u8;
        if let Some(x) = hex_char_to_u8(&hex2[0]) {
            r = x << 4;
            if let Some(y) = hex_char_to_u8(&hex2[1]) {
                r += y;
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// ```
    /// // converts a str of hex to vec of u8
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(hex_str_to_u8_vec("AABBCC"),Some(vec![0xAAu8,0xBB,0xCC]));
    /// assert_eq!(hex_str_to_u8_vec("NOPE"),None);
    /// ```
    #[inline]
    pub fn hex_str_to_u8_vec(hexstr: &str) -> Option<Vec<u8>> {
        let mut ca: HexArray = ['0', '0'];
        let mut ct = 0;

        let mut v: Vec<u8> = Vec::with_capacity((hexstr.chars().count() / 2) + 1);

        if hexstr.len() % 2 != 0 {
            ct = 1;
            ca[0] = '0';
        }
        let mut crs = hexstr.chars();
        while let Some(c) = crs.next() {
            match ct {
                0 => {
                    ca[0] = c;
                    ct = 1
                }
                1 => {
                    ca[1] = c;
                    ct = 0;

                    if let Some(b) = hex_chars_to_u8(&ca) {
                        v.push(b);
                    } else {
                        return None;
                    }
                }
                _ => {}
            }
        }

        Some(v)
    }

    /// ```
    /// // i16 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i16be_to_hex(0x4FFFi16), "4FFF");
    /// ```
    #[inline]
    pub fn i16be_to_hex(i: i16) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i16 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i16le_to_hex(0x4FFFi16), "FF4F");
    /// ```
    #[inline]
    pub fn i16le_to_hex(i: i16) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// i16 to native endian hex
    ///
    #[inline]
    pub fn i16ne_to_hex(i: i16) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u16 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u16be_to_hex(0x4FFFu16), "4FFF");
    /// ```
    #[inline]
    pub fn u16be_to_hex(i: u16) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u16 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u16le_to_hex(0x4FFFu16), "FF4F");
    /// ```
    #[inline]
    pub fn u16le_to_hex(i: u16) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// u16 to native endian hex
    ///
    #[inline]
    pub fn u16ne_to_hex(i: u16) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i32 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i32be_to_hex(0x4FFFFFFFi32), "4FFFFFFF");
    /// ```
    #[inline]
    pub fn i32be_to_hex(i: i32) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i32 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i32le_to_hex(0x4FFFFFFFi32), "FFFFFF4F");
    /// ```
    #[inline]
    pub fn i32le_to_hex(i: i32) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// i32 to native endian hex
    ///
    #[inline]
    pub fn i32ne_to_hex(i: i32) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u32 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u32be_to_hex(0x4FFFFFFFu32), "4FFFFFFF");
    /// ```
    #[inline]
    pub fn u32be_to_hex(i: u32) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u32 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u32le_to_hex(0x4FFFFFFFu32), "FFFFFF4F");
    /// ```
    #[inline]
    pub fn u32le_to_hex(i: u32) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// u32 to native endian hex
    ///
    #[inline]
    pub fn u32ne_to_hex(i: u32) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i64 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i64be_to_hex(0x4FFFFFFFFFFFFFFFi64), "4FFFFFFFFFFFFFFF");
    /// ```
    #[inline]
    pub fn i64be_to_hex(i: i64) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i64 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i64le_to_hex(0x4FFFFFFFFFFFFFFFi64), "FFFFFFFFFFFFFF4F");
    /// ```
    #[inline]
    pub fn i64le_to_hex(i: i64) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// i64 to native endian hex
    ///
    #[inline]
    pub fn i64ne_to_hex(i: i64) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u64 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u64be_to_hex(0x4FFFFFFFFFFFFFFFu64), "4FFFFFFFFFFFFFFF");
    /// ```
    #[inline]
    pub fn u64be_to_hex(i: u64) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u64 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u64le_to_hex(0x4FFFFFFFFFFFFFFFu64), "FFFFFFFFFFFFFF4F");
    /// ```
    #[inline]
    pub fn u64le_to_hex(i: u64) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// u64 to native endian hex
    ///
    #[inline]
    pub fn u64ne_to_hex(i: u64) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i128 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i128be_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFi128), "4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    /// ```
    #[inline]
    pub fn i128be_to_hex(i: i128) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // i128 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(i128le_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFi128), "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4F");
    /// ```
    #[inline]
    pub fn i128le_to_hex(i: i128) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// u64 to native endian hex
    ///
    #[inline]
    pub fn i128ne_to_hex(i: i128) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u128 to big endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u128be_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128), "4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    /// ```
    #[inline]
    pub fn u128be_to_hex(i: u128) -> String {
        let r = i
            .to_be_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    /// ```
    /// // u128 to little endian hex
    ///
    /// use cj_common::prelude::*;
    ///
    /// assert_eq!(u128le_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128), "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4F");
    /// ```
    #[inline]
    pub fn u128le_to_hex(i: u128) -> String {
        let r = i
            .to_le_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    ///
    /// u64 to native endian hex
    ///
    #[inline]
    pub fn u128ne_to_hex(i: u128) -> String {
        let r = i
            .to_ne_bytes()
            .iter()
            .map(|f| HEX_TABLE[*f as usize])
            .collect::<String>();
        r
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_u8_to_hex_str() {
            assert_eq!(u8_to_hex_str(&0xD1), "D1");
        }

        #[test]
        fn test_u8_to_hex() {
            assert_eq!(u8_to_hex(&0xD1), "D1".to_string());
        }

        #[test]
        fn test_u8_array_to_hex() {
            let array = [0xA0, 0xA1, 0xA2];
            assert_eq!(u8_array_to_hex(&array), "A0A1A2");
        }

        #[test]
        fn test_hex_char_to_u8() {
            assert_eq!(hex_char_to_u8(&'A'), Some(0x0A));
            assert_eq!(hex_char_to_u8(&'G'), None);
        }

        #[test]
        fn test_hex_str_to_u8() {
            assert_eq!(hex_str_to_u8("AB"), Some(0xAB));
            assert_eq!(hex_str_to_u8("G"), None);
        }

        #[test]
        fn test_hex_chars_to_u8() {
            assert_eq!(hex_chars_to_u8(&['A', 'B']), Some(0xAB));
            assert_eq!(hex_chars_to_u8(&['N', 'O']), None);
        }

        #[test]
        fn test_hex_str_to_u8_vec() {
            assert_eq!(hex_str_to_u8_vec("AABBCC"), Some(vec![0xAAu8, 0xBB, 0xCC]));
            assert_eq!(hex_str_to_u8_vec("NOPE"), None);
        }

        #[test]
        fn test_i16be_to_hex() {
            assert_eq!(i16be_to_hex(0x4FFFi16), "4FFF");
        }

        #[test]
        fn test_i16le_to_hex() {
            assert_eq!(i16le_to_hex(0x4FFFi16), "FF4F");
        }

        #[test]
        fn test_u16be_to_hex() {
            assert_eq!(u16be_to_hex(0x4FFFu16), "4FFF");
        }

        #[test]
        fn test_u16le_to_hex() {
            assert_eq!(u16le_to_hex(0x4FFFu16), "FF4F");
        }

        #[test]
        fn test_i32be_to_hex() {
            assert_eq!(i32be_to_hex(0x4FFFFFFFi32), "4FFFFFFF");
        }

        #[test]
        fn test_i32le_to_hex() {
            assert_eq!(i32le_to_hex(0x4FFFFFFFi32), "FFFFFF4F");
        }

        #[test]
        fn test_u32be_to_hex() {
            assert_eq!(u32be_to_hex(0x4FFFFFFFu32), "4FFFFFFF");
        }

        #[test]
        fn test_u32le_to_hex() {
            assert_eq!(u32le_to_hex(0x4FFFFFFFu32), "FFFFFF4F");
        }

        #[test]
        fn test_i64be_to_hex() {
            assert_eq!(i64be_to_hex(0x4FFFFFFFFFFFFFFFi64), "4FFFFFFFFFFFFFFF");
        }

        #[test]
        fn test_i64le_to_hex() {
            assert_eq!(i64le_to_hex(0x4FFFFFFFFFFFFFFFi64), "FFFFFFFFFFFFFF4F");
        }

        #[test]
        fn test_u64be_to_hex() {
            assert_eq!(u64be_to_hex(0x4FFFFFFFFFFFFFFFu64), "4FFFFFFFFFFFFFFF");
        }

        #[test]
        fn test_u64le_to_hex() {
            assert_eq!(u64le_to_hex(0x4FFFFFFFFFFFFFFFu64), "FFFFFFFFFFFFFF4F");
        }

        #[test]
        fn test_i128be_to_hex() {
            assert_eq!(
                i128be_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFi128),
                "4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
            );
        }

        #[test]
        fn test_i128le_to_hex() {
            assert_eq!(
                i128le_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFi128),
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4F"
            );
        }

        #[test]
        fn test_u128be_to_hex() {
            assert_eq!(
                u128be_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128),
                "4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
            );
        }

        #[test]
        fn test_u128le_to_hex() {
            assert_eq!(
                u128le_to_hex(0x4FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128),
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF4F"
            );
        }
    }
}
