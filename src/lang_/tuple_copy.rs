#[test]
fn test_tuple_copy_trait() {
    struct A(i32, i32); //自动实现 Copy trait

    let b1: (i32, i32) = (1, 2);
    let b2 = b1;//自动实现 Copy trait
    print!("{:?}", b1);
    let _ = b2;
}