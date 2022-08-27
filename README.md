# cj_common

Collection of common functions used for other projects. Additional functionality added as more projects are spun
up.

cj_binary
---

- b64 - structs, methods and traits for working with b64 encoding/decoding

```rust
fn main() {
    let mut s2 = String::new();
    for c in "Many hands make light work.".iter_base64() {
        s2.push(c);
    }
    assert_eq!(s2.as_str(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
}
```

```rust
fn main() {
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

```rust
fn main() {
    let x = 0x1F2i64;
    let s = x.to_hex_be();
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
    let mut s = String::new();
    for c in "Many hands make light work.".iter_hex() {
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
    let x = 0b00000010u8;
    assert_eq!(x.get_bit(1), true);
}
```

```rust
fn main() {
    let mut x = 0b00000000u8;
    x.set_bit(1, true);
    assert_eq!(x, 0b00000010u8);
}
```

```rust
fn main() {
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

cj_helpers
---

- in_set - structs, methods and traits for checking if values are within a given range.

```rust
fn main() {
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
}
```


   