#[test]
fn test_default_construct() {
    struct A {
        name: String,
        number: i32,
    }
    impl Default for A {
        fn default() -> Self {
            Self {
                name: "name".to_string(),
                number: 10,
            }
        }
    }

    let a1 = A { name: "".to_string(), ..Default::default() };
    assert_eq!(a1.number, 10);

    let a2 = A { name: "".to_string(), ..A::default() };
    assert_eq!(a2.number, 10);
}