# cj_common

Collection of common functions used for other projects. Additional functionality will be added as more projects are spun
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


   