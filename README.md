# cj_common

[![Rust](https://github.com/cubicle-jockey/cj_common/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/cj_common/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/cj_common/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/cj_common/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/cj_common.svg)](https://crates.io/crates/cj_common)
[![API](https://docs.rs/cj_common/badge.svg)](https://docs.rs/cj_common)

Collection of common functions used for other projects. Additional functionality added as more projects are spun
up.

Current features relate to:

```text
* Base64 encoding/decoding
* Hex encoding/decoding
* Bit manipulation
* In-set checking (values within a set of ranges)
```

cj_binary
---

- b64 - structs, methods and traits for working with b64 encoding/decoding

```rust
fn main() {
    use cj_common::prelude::*;

    let mut s2 = String::new();
    for c in "Many hands make light work.".iter_to_b64() {
        s2.push(c);
    }
    assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
}
```

```rust
fn main() {
    use cj_common::prelude::*;

    let mut v = Vec::new();
    for b in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
        v.push(b);
    }
    assert!(v.len() > 0);
    let r = String::from_utf8_lossy(v.as_slice());
    let s = "Many hands make light work.";
    assert_eq!(r.to_string().as_str(), s);
}
```

- hex - structs, methods and traits for working with hex encoding/decoding
  <br>
  <br>
  <i>Note that uppercase hex is the default output. Lowercase is supported too
  and most methods have a corresponding _low() implementation. For example, calling
  to_hex_be_low() instead of to_hex_be() will result in lowercase output</i>

```rust
fn main() {
    use cj_common::prelude::*;

    let x = 0x1F2i64;
    let s = x.to_hex_be();
    x.to_hex_be_low();
    let y: Option<i64> = i64::from_hex_be(s.as_str());
    assert!(y.is_some());
    assert_eq!(x, y.unwrap());

    let x = 0x1F2i64;
    let s = x.to_hex_le();
    let y: Option<i64> = i64::from_hex_le(s.as_str());
    assert!(y.is_some());
    assert_eq!(x, y.unwrap());
}
```

```rust
fn main() {
    use cj_common::prelude::*;

    let mut s = String::new();
    for c in "Many hands make light work.".iter_to_hex() {
        s.push_str(c);
    }
    assert_eq!(
        s.as_str(),
        "4D616E792068616E6473206D616B65206C6967687420776F726B2E"
    );

    let mut v = Vec::new();
    for b in s.as_str().iter_hex_to_byte() {
        v.push(b);
    }
    let s2 = String::from_utf8_lossy(v.as_slice()).to_string();
    assert_eq!(s2.as_str(), "Many hands make light work.");
}
```

- bitbuf - structs, methods and traits for getting/setting bits at given positions of the implemented types

```rust
fn main() {
    use cj_common::prelude::*;

    let x = 0b00000010u8;
    assert_eq!(x.get_bit(1), true);
}
```

```rust
fn main() {
    use cj_common::prelude::*;

    let mut x = 0b00000000u8;
    x.set_bit(1, true);
    assert_eq!(x, 0b00000010u8);
}
```

```rust
fn main() {
    use cj_common::prelude::*;

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
```

```rust
fn main() {
    use cj_common::prelude::*;

    let x = vec![0xABu8, 0xAB, 0xAB];
    let mut v = Vec::new();
    for i in x.iter_to_bit() {
        v.push(i);
    }

    assert_eq!(
        v.as_slice(),
        &[
            true, true, false, true, false, true, false, true,
            true, true, false, true, false, true, false, true,
            true, true, false, true, false, true, false, true
        ]
    );

    let x = [2u128, 2, 2];
    for i in x.as_slice().iter_to_bit().enumerate() {
        match i.0 {
            1 | 129 | 257 => assert_eq!(i.1, true),
            _ => assert_eq!(i.1, false),
        }
    }
}
```

```rust
fn main() {
    use cj_common::prelude::*;
    // mask examples
    let mask = 0b00011010u8;
    let byte = 0b01011010u8;
    assert_eq!(byte.matches_mask(&mask), true);
    assert_eq!(mask.as_mask_matches(&byte), true);

    let read_permission = 0b00000001u8;
    let write_permission = 0b00000010u8;
    let mod_permission = 0b00000100u8;
    let del_permission = 0b00001000u8;
    let full_permission = read_permission + write_permission + mod_permission + del_permission;
    let user = read_permission + write_permission;
    let moderator = user + mod_permission;
    let admin = full_permission;

    let fred = user;
    let jane = moderator;
    assert_eq!(fred.matches_mask(&read_permission), true);
    assert_eq!(fred.matches_mask(&write_permission), true);
    assert_eq!(fred.matches_mask(&moderator), false);
    assert_eq!(user.as_mask_matches(&jane), true);
    assert_eq!(admin.as_mask_matches(&jane), false);
}
```

cj_helpers
---

- in_set - structs, methods and traits for checking if values are within a given set of ranges.

```rust
fn main() {
    use cj_common::prelude::*;

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
```


   