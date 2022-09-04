//! Collection of common functions used by cubicle-jockey projects
//!
//! Current features relate to:
//! * Base64 encoding/decoding
//! * Hex encoding/decoding
//! * Bit manipulation
//! * In-set checking (values within a set of ranges)
//!
//! # Quick Start
//!
//! The simplest way to get started is by using prelude to import all the main functionality
//!
//! ```
//! use cj_common::prelude::*;
//!
//! // base64 examples
//! let s = "Many hands make light work.".to_b64_string();
//! assert_eq!(s.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
//!
//! if let Some(v) = b64_to_bytes("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu") {
//!     let r = String::from_utf8_lossy(v.as_slice()).to_string();
//!     assert_eq!(r.as_str(), "Many hands make light work.");
//! }
//!
//! // iter example
//! let mut s = String::new();
//! for c in "Many hands make light work.".iter_to_b64() {
//!     s.push(c);
//! }
//! assert_eq!(s.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
//!
//! let mut v = Vec::new();
//! for b in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
//!     v.push(b);
//! }
//! let r = String::from_utf8_lossy(v.as_slice()).to_string();
//! assert_eq!(r.as_str(), "Many hands make light work.");
//!
//! // the above are str examples, but it works the same for u8
//! let s = vec![1u8,2,3,4,5,6].to_b64_string();
//! let iter = [1u8,2,3,4,5,6].as_slice().iter_to_b64();
//!
//!
//! // hex examples
//! let mut s = String::new();
//! for c in "Many hands make light work.".iter_to_hex() {
//!     s.push_str(c);
//! }
//! assert_eq!(s.as_str(), "4D616E792068616E6473206D616B65206C6967687420776F726B2E");
//!
//! let mut v = Vec::new();
//! for b in "4D616E792068616E6473206D616B65206C6967687420776F726B2E".iter_hex_to_byte() {
//!     v.push(b);
//! }
//! let s = String::from_utf8_lossy(v.as_slice()).to_string();
//! assert_eq!(s.as_str(), "Many hands make light work.");
//!
//!
//! // bitbuf examples
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
//!         true, true, false, true, false, true, false, true, true, true, false, true, false,
//!         true, false, true, true, true, false, true, false, true, false, true
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
//!
//! // in_set example
//! let list = "lmnop";
//! for c in list.chars() {
//!     assert_eq!(c.in_range('k'..'q'), true);
//!     assert_eq!(
//!         c.in_set(
//!             [
//!                 ('k'..='l').into(),                // RangeInclusive
//!                 ('m'..'n').into(),                 // Range
//!                 ('n'..='p').into(),                // RangeInclusive
//!                 ['a', 'b', 'c'].as_slice().into(), // Slice
//!                 "test123".into(),                  // str
//!             ]
//!             .as_slice()
//!         ),
//!         true
//!     );
//!     assert_eq!(c.in_range('w'..'z'), false);
//! }
//!
//! let list = [1_000, 10_000, 100_000_000];
//! for n in list {
//!     assert_eq!(n.in_range(1..200_000_000), true);
//!     assert_eq!(
//!         n.in_set(
//!             [
//!                 (1..=10).into(),                 // RangeInclusive
//!                 (500..2_000).into(),             // Range
//!                 (9_999..=100_000_000).into(),    // RangeInclusive
//!                 [30, 90, 700].as_slice().into()  // Slice
//!             ]
//!             .as_slice()
//!         ),
//!         true
//!     );
//!     assert_eq!(n.in_range(1_000_000_000..1_000_000_001), false);
//! }
//! ```

/// b64, hex and bitbuf
pub mod cj_binary;

/// in_set
pub mod cj_helpers;

/// easiest way to import all functionality
pub mod prelude {
    pub use crate::cj_binary::b64::*;
    pub use crate::cj_binary::bitbuf::*;
    pub use crate::cj_binary::hex::*;
    pub use crate::cj_helpers::in_set::*;
}
