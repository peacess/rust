#[test]
fn test_retrun_op() {
    let l = last_char("sdf");
    println!("{:?}", l);
}

fn last_char(s: &str) -> Option<char> {
    s.lines().next()?.chars().last()
}
