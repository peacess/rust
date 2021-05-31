#[test]
fn test_sized() {
    struct Foo<T>(T);
    struct Bar<T: ?Sized>(T);
    // struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
    struct BarUse(Bar<[i32]>); // OK

    let v: Box::<BarUse>;
}