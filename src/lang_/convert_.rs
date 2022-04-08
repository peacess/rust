
#[test]
fn test_isize_usize(){
    let i_value = -10;
    let u_value = i_value as usize;
    assert_ne!(u_value, 10);

    let i_value = -1;
    let u_value = i_value as usize;
    assert_ne!(u_value, 1);
}