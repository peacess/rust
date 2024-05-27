#[test]
fn test_match_other() {
    let status = 10;
    match status {
        1 => println!("1"),
        other => println!("{}", other),
    }

    match status {
        1 => println!("1"),
        other @ _ => println!("{}", other),
    }
}
