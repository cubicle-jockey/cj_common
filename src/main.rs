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