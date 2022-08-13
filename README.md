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
    use cj_common::prelude::CjFromBase64Iter;
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
fn test_i64() {
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

*            



   