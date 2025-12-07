# cj_common

[![Rust](https://github.com/cubicle-jockey/cj_common/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/cj_common/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/cj_common/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/cj_common/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/cj_common.svg)](https://crates.io/crates/cj_common)
[![API](https://docs.rs/cj_common/badge.svg)](https://docs.rs/cj_common)

A comprehensive Rust library providing essential utilities for encoding, bit manipulation, and data validation. This
crate offers high-performance implementations of commonly needed functionality with both direct conversion methods and
efficient iterator-based approaches.

## Features

- **🔐 Base64 Encoding/Decoding** - Complete Base64 support with string conversion and streaming iterators
- **🔢 Hexadecimal Encoding/Decoding** - Full hex support with uppercase/lowercase options and iterator interfaces
- **⚡ Bit Manipulation** - Efficient bit-level operations with get/set functionality and bit iteration
- **📊 Range Validation** - Flexible in-set checking for values within ranges, slices, and collections
- **🕒 Time Utilities (feature: `timext`)** - `OffsetDateTimeExt` with helpers like `to_primitive()` to get a
  `PrimitiveDateTime`
- **🚀 High Performance** - Optimized implementations with zero-copy iterators where possible
- **🔧 Easy Integration** - Simple prelude module for importing all functionality

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cj_common = "1.2.0"
```

For async channel functionality, enable the `channel` feature:

```toml
[dependencies]
cj_common = { version = "1.2.0", features = ["channel"] }
```

For time utilities (e.g., `OffsetDateTimeExt`), enable the `timext` feature:

```toml
[dependencies]
cj_common = { version = "1.2.0", features = ["timext"] }
```

## Quick Start

The easiest way to get started is by importing the prelude module:

```rust
use cj_common::prelude::*;

fn quick_start_example() {
    // Base64 encoding
    let encoded = "Hello, World!".to_b64_string();
    println!("Encoded: {}", encoded);

    // Base64 decoding
    if let Some(decoded_bytes) = b64_to_bytes(&encoded) {
        let decoded = String::from_utf8_lossy(&decoded_bytes);
        println!("Decoded: {}", decoded);
    }

    // Hex encoding
    let mut hex_string = String::new();
    for hex_pair in "Hello".iter_to_hex() {
        hex_string.push_str(hex_pair);
    }
    println!("Hex: {}", hex_string);

    // Bit manipulation
    let mut value = 0u8;
    value.set_bit(3, true);  // Set bit 3
    assert_eq!(value.get_bit(3), true);

    // Range checking
    assert_eq!('m'.in_range('a'..'z'), true);
}

```

## API Documentation

### 🔐 Base64 (`cj_binary::b64`)

Complete Base64 encoding and decoding with multiple approaches:

#### Direct Conversion

```rust
use cj_common::prelude::*;

fn base64_direct_conversion_example() {
    // String to Base64
    let encoded = "Many hands make light work.".to_b64_string();
    assert_eq!(encoded, "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");

    // Base64 to bytes
    if let Some(decoded) = b64_to_bytes("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu") {
        let text = String::from_utf8_lossy(&decoded);
        assert_eq!(text, "Many hands make light work.");
    }
}
```

#### Iterator-Based Approach

```rust
use cj_common::prelude::*;

fn base64_iterator_example() {
    // Encode using iterator
    let mut encoded = String::new();
    for c in "Many hands make light work.".iter_to_b64() {
        encoded.push(c);
    }
    assert_eq!(encoded, "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");

    // Decode using iterator
    let mut decoded = Vec::new();
    for byte in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
        decoded.push(byte);
    }
    let text = String::from_utf8_lossy(&decoded);
    assert_eq!(text, "Many hands make light work.");
}
```

### 🕒 Time Utilities (`cj_helpers::timext`, feature: `timext`)

Helpers for working with the `time` crate types. Currently provides `OffsetDateTimeExt` with methods for converting or
deriving related types.

- `OffsetDateTimeExt::to_primitive()` — strips the offset and returns the wall-clock `PrimitiveDateTime`.

Enable the feature and use it like this:

```toml
cj_common = { version = "1.2.0", features = ["timext"] }
```

```rust
use time::OffsetDateTime;
use cj_common::cj_helpers::timext::OffsetDateTimeExt; // trait

fn timext_example() {
    // Example timestamp: adjust as needed
    let dt = OffsetDateTime::from_unix_timestamp(1_766_496_840).unwrap();
    let primitive = dt.to_primitive();
    // primitive is a time::PrimitiveDateTime
    println!("{} {}", primitive.date(), primitive.time());
}
```

### 🔢 Hexadecimal (`cj_binary::hex`)

Flexible hexadecimal encoding and decoding with case options:

> **Note:** Uppercase hex is the default output. Most methods have a corresponding `_low()` variant for lowercase
> output (e.g., `to_hex_be_low()` instead of `to_hex()`).

#### Basic Usage

```rust
use cj_common::prelude::*;

fn hex_basic_usage_example() {
    // String to hex
    let mut hex = String::new();
    for hex_pair in "Hello".iter_to_hex() {
        hex.push_str(hex_pair);
    }
    println!("Hex: {}", hex);

    // Numeric values
    let value = 0x1F2i64;
    let hex_be = value.to_hex_be();      // Big-endian
    let hex_le = value.to_hex_le();      // Little-endian
    let hex_be_low = value.to_hex_be_low(); // Lowercase
}
```

#### Iterator-Based Approach

```rust
use cj_common::prelude::*;

fn hex_iterator_example() {
    // Encode using iterator
    let mut hex_string = String::new();
    for hex_pair in "Many hands make light work.".iter_to_hex() {
        hex_string.push_str(hex_pair);
    }
    assert_eq!(hex_string, "4D616E792068616E6473206D616B65206C6967687420776F726B2E");

    // Decode using iterator
    let mut decoded = Vec::new();
    for byte in "4D616E792068616E6473206D616B65206C6967687420776F726B2E".iter_hex_to_byte() {
        decoded.push(byte);
    }
    let text = String::from_utf8_lossy(&decoded);
    assert_eq!(text, "Many hands make light work.");
}
```

### ⚡ Bit Manipulation (`cj_binary::bitbuf`)

Efficient bit-level operations with comprehensive functionality:

#### Basic Bit Operations

```rust
use cj_common::prelude::*;

fn basic_bit_operations_example() {
    // Get and set individual bits
    let x = 0b00000010u8;
    assert_eq!(x.get_bit(1), true);

    let mut x = 0b00000000u8;
    x.set_bit(1, true);
    assert_eq!(x, 0b00000010u8);
}
```

#### Bit Iteration

```rust
use cj_common::prelude::*;

fn bit_iteration_example() {
    // Iterate over bits in a single value
    let x = 0xABu8;
    let mut bits = Vec::new();
    for bit in x.bit_iter() {
        bits.push(bit);
    }
    assert_eq!(bits, [true, true, false, true, false, true, false, true]);

    // Iterate over bits in a collection
    let data = vec![0xABu8, 0xAB, 0xAB];
    let mut all_bits = Vec::new();
    for bit in data.iter_to_bit() {
        all_bits.push(bit);
    }
    // Results in 24 bits total (3 bytes × 8 bits each)
}
```

#### Advanced Bit Operations

```rust
use cj_common::prelude::*;

fn advanced_bit_operations_example() {
    // Work with different integer types
    let x = [2u128, 2, 2];
    for (index, bit) in x.as_slice().iter_to_bit().enumerate() {
        match index {
            1 | 129 | 257 => assert_eq!(bit, true),  // Bit position 1 in each u128
            _ => assert_eq!(bit, false),
        }
    }
}
```

### 📊 Range Validation (`cj_helpers::in_set`)

Flexible validation for checking if values exist within specified ranges or collections:

#### Basic Range Checking

```rust
use cj_common::prelude::*;

fn basic_range_checking_example() {
    // Simple range checking
    assert_eq!('m'.in_range('a'..'z'), true);
    assert_eq!(15.in_range(10..20), true);
    assert_eq!(25.in_range(10..20), false);
}
```

#### Complex Set Validation

```rust
use cj_common::prelude::*;

fn complex_set_validation_example() {
    // Character validation with multiple criteria
    let test_chars = "lmnop";
    for c in test_chars.chars() {
        assert_eq!(c.in_range('k'..'q'), true);

        // Check against multiple sets
        assert_eq!(
            c.in_set([
                ('k'..='l').into(),                // RangeInclusive
                ('m'..'n').into(),                 // Range
                ('n'..='p').into(),                // RangeInclusive
                ['a', 'b', 'c'].as_slice().into(), // Slice
                "test123".into(),                  // str
            ].as_slice()),
            true
        );
    }

    // Numeric validation
    let numbers = [1_000, 10_000, 100_000_000];
    for n in numbers {
        assert_eq!(n.in_range(1..200_000_000), true);

        assert_eq!(
            n.in_set([
                (1..=10).into(),                 // RangeInclusive
                (500..2_000).into(),             // Range
                (9_999..=100_000_000).into(),    // RangeInclusive
                [30, 90, 700].as_slice().into()  // Slice
            ].as_slice()),
            true
        );
    }
}
```

## Performance

This crate is designed with performance in mind:

- **Zero-copy iterators** where possible to minimize memory allocations
- **Optimized algorithms** for encoding/decoding operations
- **Efficient bit manipulation** using native CPU instructions
- **Minimal dependencies** to reduce compilation time and binary size

## Benchmarks

This crate includes comprehensive benchmarks to measure performance across all major functionality. The benchmarks
cover:

- **Base64 encoding/decoding** - Both direct conversion and iterator-based approaches
- **Hex encoding/decoding** - String and binary data with various sizes
- **Bit manipulation** - Get/set operations and bit iteration across different integer types
- **Range validation** - Simple range checks and complex set operations

### Running Benchmarks

To run the benchmarks:

```bash
cargo bench
```

This will generate detailed performance reports and HTML output (if available) showing timing comparisons across
different operations and data sizes.

### Benchmark Results

The benchmarks test various scenarios including:

- Small vs. large data sets
- String vs. binary data encoding
- Direct function calls vs. iterator-based processing
- Different integer types for bit operations

Results are displayed with statistical analysis including confidence intervals and outlier detection.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE_APACHE](LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE_MIT](LICENSE_MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to
discuss what you would like to change.

## Links

- [Documentation](https://docs.rs/cj_common)
- [Crates.io](https://crates.io/crates/cj_common)
- [Repository](https://github.com/cubicle-jockey/cj_common)