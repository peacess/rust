//! tuple struct是字段名是以下标自动命名的struct
//! tuple 不是struct, 不能定义类型名字，多用于函数多参返回值。会自动实现Copy trait等"strut"不会实现的功能
//! enum 存储空间是最大的那个字段 + discriminant(默认为32的整数，如果只有一个字段，这个discriminant会被省略。这个是用于判断当前enum是那一个字段)
//! union 是与c兼容的类型，是unsafe的，这个类型有很多限制（如字段要实现Copy trait等），建议只使用在与C兼容

use std::mem::{size_of, size_of_val};
pub struct S1 {
    name: i32,
    number: i32,
}

/// s2
pub struct S2(i32, i32);

/// size of ： 8,  最大字段4 + 默认的discriminant 4 = 8
pub enum E1 {
    Name(i32),
    Number(i32),
}

/// size of 为最大的字段 4, 没有附加信息
pub union U1 {
    name: i32,
    number: i32,
}

#[test]
fn test_s_e_u() {
    let t = (1i32, 3u32);
    println!(
        "struct: {}\ntuple struct:{}\nenum:{}\nunion:{}\ntuple:{}",
        size_of::<S1>(),
        size_of::<S2>(),
        size_of::<E1>(),
        size_of::<U1>(),
        size_of_val(&t)
    );
}
