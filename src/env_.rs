

#[test]
fn test_env_path(){
    let ex_path = std::env::current_exe().expect("exe path");
    println!("{}", ex_path.to_str().expect(""));
    let ex_path = ex_path.parent().expect("execute path");
    println!("{}", ex_path.to_str().expect(""));
    // println!("{}", env!("CARGO_BUILD_TARGET_DIR"));
}