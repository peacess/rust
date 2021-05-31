#[test]
fn test_char() {
    let chars = "é".chars().collect::<Vec<_>>();
// U+00e9: 'latin small letter e with acute'
    assert_eq!(vec!['\u{00e9}'], chars);
    let chars2 = "é".chars().collect::<Vec<_>>();
// U+0065: 'latin small letter e',U+0301: 'combining acute accent'
    assert_eq!(vec!['\u{0065}', '\u{0301}'], chars2);
}
#[test]
fn test_use_as() {
    struct A{ };
    use A as VV;
    type V2 = A;
    impl V2 {
        pub fn okok(){}
    }
    impl VV {
        pub fn ok(){}
    }
}

#[test]
fn test_sized() {
    trait T {
        fn f(&self);
        fn f2(&self) {}
        // fn f3(self) {}
        fn f4(self) where Self: Sized {}
    }
    struct Ts;
    impl T for Ts{
        fn f(&self) {
            println!("");
        }
    }
    let ts = Ts{};
    ts.f4();
    let t:&T = &Ts{};
    t.f2();
    // t.f4();
}