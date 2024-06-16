#[test]
fn test_args() {
    let i = 6;
    println!("{:p}", &i);
    ff(i);
}

fn ff(i: u128) {
    println!("{:p}", &i);
}
