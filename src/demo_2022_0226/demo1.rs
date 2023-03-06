// #![feature(raw)]

use std::io::{Error, ErrorKind};

struct A<'a> {
    name: &'a str,
}

fn main() {
    let mut a = A { name: "I am here" };
    let t: *mut A = &mut a as *mut A;
    println!("{:p}", t);
    // std::raw::
}