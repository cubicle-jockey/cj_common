use std::time::Instant;

use cj_common::prelude::*;

// main.rs is only meant to help with testing/stepping through the code.
fn main() {
    b64_test5();
    b64_test6();
    b64_test7();
    b64_test8();
    b64_test9();
    b64_test10();
    b64_test11();
    b64_test12();
    b64_test_iter();
    b64_test_iter2();
    b64_test_iter4();
    b64_test_iter5();

    perf_test_1();
    perf_test_2();

    hex_test1();

    bit_iter_test1();

    hex_iter_test1();
    hex_iter_test2();

    in_set_test();

    perf_in_set_test_1();
    perf_in_set_test_2();

    //let x = ((0..5), (6..7), 8, 9);
    //x.contains(3);

    //_remove_me();
}

fn _remove_me() {
    let mut v8 = Vec::new();
    let mut v16 = Vec::new();
    let mut v32 = Vec::new();
    let mut v64 = Vec::new();
    let mut v128 = Vec::new();
    let mut x = 1u128;
    let mut inx = 1;

    //r.

    loop {
        match inx {
            1..=8 => {
                v8.push(x as u8);
                v16.push(x as u16);
                v32.push(x as u32);
                v64.push(x as u64);
                v128.push(x as u128);
            }
            9..=16 => {
                v16.push(x as u16);
                v32.push(x as u32);
                v64.push(x as u64);
                v128.push(x as u128);
            }
            17..=32 => {
                v32.push(x as u32);
                v64.push(x as u64);
                v128.push(x as u128);
            }
            33..=64 => {
                v64.push(x as u64);
                v128.push(x as u128);
            }
            65..=128 => {
                v128.push(x as u128);
            }
            _ => {
                break;
            }
        }
        x *= 2;
        inx += 1;
    }

    println!("{v8:?}");
    println!("{v16:?}");
    println!("{v32:?}");
    println!("{v64:?}");
    println!("{v128:?}");
}

fn b64_test5() {
    let s = "M".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test6() {
    let s = "Ma".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test7() {
    let s = "Man".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test8() {
    let s = "Many".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test9() {
    let s = "Many ".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test10() {
    let s = "Many h".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test11() {
    let s = "Many ha".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test12() {
    let s = "Many han".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hand".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands ".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands m".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands ma".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands mak".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make ".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make l".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make li".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make lig".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make ligh".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light ".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light w".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light wo".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light wor".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work.".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work..".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work...".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work...8".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work...86".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work...867".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);

    let s = "Many hands make light work...8675".as_bytes();
    let s2 = bytes_to_b64(s);
    println!("{}", s2);
}

fn b64_test_iter() {
    let s = "Many hands make light work...8675".as_bytes();
    let mut s2 = String::new();
    //let mut bs = s[..].iter();
    let mut it = ToBase64Iter::new(s[..].iter());
    while let Some(c) = it.next() {
        s2.push(c);
    }
    println!("{}", s2);
}

fn b64_test_iter2() {
    let s = "Many hands make light work...8675".as_bytes();
    let mut s2 = String::new();

    let mut it = s.iter_to_b64();
    while let Some(c) = it.next() {
        s2.push(c);
    }
    println!("{}", s2);
}

fn b64_test_iter4() {
    let s = "Many hands make light work...8675".as_bytes();
    let mut s2 = String::new();

    for c in s.iter_to_b64() {
        s2.push(c);
    }
    println!("{}", s2);
}

fn b64_test_iter5() {
    let s = "Many hands make light work...8675".as_bytes().to_vec();
    let mut s2 = String::new();

    for c in s.iter_to_b64() {
        s2.push(c);
    }
    println!("{}", s2);
}

fn perf_test_1() {
    let s = "Many hands make light work...8675".as_bytes().to_vec();
    let mut s2 = String::with_capacity(1024);
    let mut ct = 0usize;
    let now = Instant::now();
    let iters = 1_000_000;
    let mut total_str_bytes = 0usize;
    for _ in 1..=iters {
        for c in s.iter_to_b64() {
            s2.push(c);
            ct += 1;
        }
        total_str_bytes += s2.len();
        s2.clear();
    }
    let elap = now.elapsed().as_millis();
    println!("{iters} in {elap}ms. total bytes {ct}, total str bytes {total_str_bytes}");
    // 122ms
}

fn perf_test_2() {
    let mut ct = 0usize;
    let iters = 1_000_000;
    let mut total_str_bytes = 0usize;
    let mut v = Vec::with_capacity(1024);
    let now = Instant::now();
    for _ in 1..=iters {
        for b in "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".iter_b64_to_byte() {
            v.push(b);
            ct += 1;
        }
        total_str_bytes += v.len();
        v.clear();
    }
    let elap = now.elapsed().as_millis();
    println!("{iters} in {elap}ms. total bytes {ct}, total str bytes {total_str_bytes}");
    // 96ms
}

fn hex_test1() {
    let s = "Many hands make light work.".as_bytes();
    let mut s2 = String::new();
    for c in s.iter_to_hex() {
        s2.push_str(c);
    }
    println!("{s2}");
    assert_eq!(
        s2.as_str(),
        "4D616E792068616E6473206D616B65206C6967687420776F726B2E"
    );
}

fn bit_iter_test1() {
    let now = Instant::now();
    let mut total = 0;
    let x = 0b01000000u8;
    for _ in 1..=10_000_000 {
        for i in x.bit_iter() {
            if i {
                total += 1;
            }
        }
    }

    let elap = now.elapsed().as_millis();
    println!("{total} found in {elap}ms");
}

fn hex_iter_test1() {
    let mut s2 = String::with_capacity(1024);
    let now = Instant::now();
    let mut total_str_bytes = 0usize;
    let iters = 1_000_000;
    for _ in 1..=iters {
        for c in "Many hands make light work.".iter_to_hex() {
            s2.push_str(c);
        }
        total_str_bytes += s2.len();
        s2.clear();
    }
    let elap = now.elapsed().as_millis();
    println!("{iters} in {elap}ms. total str bytes {total_str_bytes}");
}

fn hex_iter_test2() {
    let mut v = Vec::with_capacity(1024);
    let now = Instant::now();
    let mut total_str_bytes = 0usize;
    let iters = 1_000_000;
    for _ in 1..=iters {
        for b in "4D616E792068616E6473206D616B65206C6967687420776F726B2E".iter_hex_to_byte() {
            v.push(b);
        }

        total_str_bytes += v.len();
        v.clear();
    }
    let elap = now.elapsed().as_millis();
    println!("{iters} in {elap}ms. total str bytes {total_str_bytes}");
}

fn in_set_test() {
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
            c.in_set([('k'..='l').into(), ('m'..'n').into(), ('n'..='p').into()].as_slice()),
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
                    (1..=10).into(),
                    (500..2_000).into(),
                    (9_999..=100_000_000).into()
                ]
                .as_slice()
            ),
            true
        );
        assert_eq!(n.in_range(1_000_000_000..1_000_000_001), false);
    }
}

fn perf_in_set_test_1() {
    let now = Instant::now();
    let mut total = 0usize;
    let list = [1_000, 10_000, 100_000_000];
    let iters = 1_000_000_000;
    for _ in 1..iters {
        for n in list {
            if n.in_set(
                [
                    (1..=10).into(),
                    (500..2_000).into(),
                    (9_999..=100_000_000).into(),
                ]
                .as_slice(),
            ) {
                total += 1;
            }
        }
    }
    let elap = now.elapsed().as_millis();
    println!("{iters} in {elap}ms. total {total}");
}

fn perf_in_set_test_2() {
    let now = Instant::now();
    let mut total = 0usize;

    let iters = 1_000_000;
    for _ in 1..iters {
        for n in "ThisIsaTest".chars() {
            if n.in_set(
                [
                    ('a'..='g').into(),
                    ('I'..'T').into(),
                    ('T'..='P').into(),
                    "is".into(),
                ]
                .as_slice(),
            ) {
                total += 1;
            }
        }
    }
    let elap = now.elapsed().as_millis();
    println!("{iters} in {elap}ms. total {total}");
}
