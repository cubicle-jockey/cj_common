use cj_common::cj_binary::hex::{hex_str_to_u8_vec, u8_array_to_hex};
use cj_common::prelude::*;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_b64_encoding(c: &mut Criterion) {
    let data = "Many hands make light work.";
    let long_data = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";
    let binary_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let large_binary_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();

    c.bench_function("b64_encode_short_string", |b| {
        b.iter(|| black_box(data).to_b64_string())
    });

    c.bench_function("b64_encode_long_string", |b| {
        b.iter(|| black_box(long_data).to_b64_string())
    });

    c.bench_function("b64_encode_binary_small", |b| {
        b.iter(|| black_box(&binary_data).to_b64_string())
    });

    c.bench_function("b64_encode_binary_large", |b| {
        b.iter(|| black_box(&large_binary_data).to_b64_string())
    });

    c.bench_function("b64_encode_iter_short", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(data.len() * 4 / 3 + 4); // Rough estimate for base64 size
            for c in black_box(data).iter_to_b64() {
                s.push(c);
            }
            s
        })
    });

    c.bench_function("b64_encode_iter_binary", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(binary_data.len() * 4 / 3 + 4); // Rough estimate for base64 size
            for c in black_box(&binary_data).iter_to_b64() {
                s.push(c);
            }
            s
        })
    });
}

fn bench_b64_decoding(c: &mut Criterion) {
    let encoded_short = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";
    let encoded_long = "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gU2VkIGRvIGVpdXNtb2QgdGVtcG9yIGluY2lkaWR1bnQgdXQgbGFib3JlIGV0IGRvbG9yZSBtYWduYSBhbGlxdWEuIFV0IGVuaW0gYWQgbWluaW0gdmVuaWFtLCBxdWlzIG5vc3RydWQgZXhlcmNpdGF0aW9uIHVsbGFtY28gbGFib3JpcyBuaXNpIHV0IGFsaXF1aXAgZXggZWEgY29tbW9kbyBjb25zZXF1YXQu";
    let encoded_binary = "AQIDBAUGBwgJCgsMDQ4PEA==";

    c.bench_function("b64_decode_short", |b| {
        b.iter(|| b64_to_bytes(black_box(encoded_short)))
    });

    c.bench_function("b64_decode_long", |b| {
        b.iter(|| b64_to_bytes(black_box(encoded_long)))
    });

    c.bench_function("b64_decode_binary", |b| {
        b.iter(|| b64_to_bytes(black_box(encoded_binary)))
    });

    c.bench_function("b64_decode_iter_short", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(encoded_short.len() * 3 / 4); // Rough estimate for base64 size
            for b in black_box(encoded_short).iter_b64_to_byte() {
                v.push(b);
            }
            v
        })
    });

    c.bench_function("b64_decode_iter_binary", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(encoded_binary.len() * 3 / 4); // Rough estimate for base64 size
            for b in black_box(encoded_binary).iter_b64_to_byte() {
                v.push(b);
            }
            v
        })
    });
}

fn bench_hex_encoding(c: &mut Criterion) {
    let data = "Many hands make light work.";
    let long_data = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";
    let binary_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let large_binary_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();

    c.bench_function("hex_encode_short_string", |b| {
        b.iter(|| u8_array_to_hex(black_box(data).as_bytes()))
    });

    c.bench_function("hex_encode_long_string", |b| {
        b.iter(|| u8_array_to_hex(black_box(long_data).as_bytes()))
    });

    c.bench_function("hex_encode_binary_small", |b| {
        b.iter(|| u8_array_to_hex(black_box(&binary_data)))
    });

    c.bench_function("hex_encode_binary_large", |b| {
        b.iter(|| u8_array_to_hex(black_box(&large_binary_data)))
    });

    c.bench_function("hex_encode_iter_short", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(data.len() * 2); // Each byte becomes two hex characters
            for hex_str in black_box(data).iter_to_hex() {
                s.push_str(hex_str);
            }
            s
        })
    });

    c.bench_function("hex_encode_iter_binary", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(binary_data.len() * 2); // Each byte becomes two hex characters
            for hex_str in black_box(&binary_data).iter_to_hex() {
                s.push_str(hex_str);
            }
            s
        })
    });
}

fn bench_hex_decoding(c: &mut Criterion) {
    let encoded_short = "4D616E792068616E6473206D616B65206C6967687420776F726B2E";
    let encoded_long = "4C6F72656D20697073756D20646F6C6F722073697420616D65742C20636F6E73656374657475722061646970697363696E6720656C69742E2053656420646F20656975736D6F642074656D706F7220696E6369646964756E74207574206C61626F726520657420646F6C6F7265206D61676E6120616C697175612E20557420656E696D206164206D696E696D2076656E69616D2C2071756973206E6F737472756420657865726369746174696F6E20756C6C616D636F206C61626F726973206E69736920757420616C697175697020657820656120636F6D6D6F646F20636F6E7365717561742E";
    let encoded_binary = "0102030405060708090A0B0C0D0E0F10";

    c.bench_function("hex_decode_short", |b| {
        b.iter(|| hex_str_to_u8_vec(black_box(encoded_short)))
    });

    c.bench_function("hex_decode_long", |b| {
        b.iter(|| hex_str_to_u8_vec(black_box(encoded_long)))
    });

    c.bench_function("hex_decode_binary", |b| {
        b.iter(|| hex_str_to_u8_vec(black_box(encoded_binary)))
    });

    c.bench_function("hex_decode_iter_short", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(encoded_short.len() / 2); // Each two hex characters become one byte
            for b in black_box(encoded_short).iter_hex_to_byte() {
                v.push(b);
            }
            v
        })
    });

    c.bench_function("hex_decode_iter_binary", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(encoded_binary.len() / 2); // Each two hex characters become one byte
            for b in black_box(encoded_binary).iter_hex_to_byte() {
                v.push(b);
            }
            v
        })
    });
}

fn bench_bitbuf_operations(c: &mut Criterion) {
    let test_byte = 0xABu8;
    let test_u16 = 0xABCDu16;
    let test_u32 = 0xABCDEF12u32;
    let test_u64 = 0xABCDEF1234567890u64;
    let test_u128 = 0xABCDEF1234567890FEDCBA0987654321u128;
    let test_vec = vec![0xABu8, 0xCD, 0xEF, 0x12];

    c.bench_function("bitbuf_get_bit_u8", |b| {
        b.iter(|| {
            for i in 0..8 {
                black_box(test_byte.get_bit(i));
            }
        })
    });

    c.bench_function("bitbuf_set_bit_u8", |b| {
        b.iter(|| {
            let mut x = 0u8;
            for i in 0..8 {
                x.set_bit(i, black_box(i % 2 == 0));
            }
            x
        })
    });

    c.bench_function("bitbuf_bit_iter_u8", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(test_byte).bit_iter() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });

    c.bench_function("bitbuf_bit_iter_u16", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(test_u16).bit_iter() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });

    c.bench_function("bitbuf_bit_iter_u32", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(test_u32).bit_iter() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });

    c.bench_function("bitbuf_bit_iter_u64", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(test_u64).bit_iter() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });

    c.bench_function("bitbuf_bit_iter_u128", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(test_u128).bit_iter() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });

    c.bench_function("bitbuf_iter_to_bit_vec", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(&test_vec).iter_to_bit() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });

    c.bench_function("bitbuf_iter_to_bit_slice", |b| {
        b.iter(|| {
            let mut count = 0;
            for bit in black_box(test_vec.as_slice()).iter_to_bit() {
                if bit {
                    count += 1;
                }
            }
            count
        })
    });
}

fn bench_in_set_operations(c: &mut Criterion) {
    let test_chars = ['a', 'k', 'm', 'p', 'z'];
    let test_numbers = [1, 500, 1000, 10000, 100000000];

    c.bench_function("in_set_char_range", |b| {
        b.iter(|| {
            for &ch in &test_chars {
                black_box(ch.in_range('k'..'q'));
            }
        })
    });

    c.bench_function("in_set_char_complex", |b| {
        b.iter(|| {
            for &ch in &test_chars {
                black_box(
                    ch.in_set(
                        [
                            ('k'..='l').into(),
                            ('m'..'n').into(),
                            ('n'..='p').into(),
                            ['a', 'b', 'c'].as_slice().into(),
                            "test123".into(),
                        ]
                        .as_slice(),
                    ),
                );
            }
        })
    });

    c.bench_function("in_set_number_range", |b| {
        b.iter(|| {
            for &num in &test_numbers {
                black_box(num.in_range(1..200_000_000));
            }
        })
    });

    c.bench_function("in_set_number_complex", |b| {
        b.iter(|| {
            for &num in &test_numbers {
                black_box(
                    num.in_set(
                        [
                            (1..=10).into(),
                            (500..2_000).into(),
                            (9_999..=100_000_000).into(),
                            [30, 90, 700].as_slice().into(),
                        ]
                        .as_slice(),
                    ),
                );
            }
        })
    });
}

criterion_group!(
    benches,
    bench_b64_encoding,
    bench_b64_decoding,
    bench_hex_encoding,
    bench_hex_decoding,
    bench_bitbuf_operations,
    bench_in_set_operations
);
criterion_main!(benches);
