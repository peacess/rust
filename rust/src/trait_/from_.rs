#[cfg(test)]
mod test {
    /// 在使用From时，如果想要reference与owner都同时工作，可以实现 AsRef。这种实现在owner时会产生一个多余的副本，所以最好的方式，两种独立实现， see [test_best_way]
    /// 建议在调用时使用 A::From 这种调用方式，函数关系明确，不会有误解

    // impl reference from
    struct Data1 {
        name: String,
    }

    struct Data2 {
        name: String,
    }

    impl From<&Data2> for Data1 {
        fn from(value: &Data2) -> Self {
            Self { name: value.name.clone() }
        }
    }

    #[test]
    fn test_compile_err() {
        let data2 = Data2 { name: "t".to_string() };
        let data1 = Data1::from(&data2); // compile ok
                                         // let data1 = Data1::from(data2);// compile error. the trait `AsRef<Data3>` is not implemented for `Data2`
                                         // let _data1: Data1 = data2.into(); // compile error.  the trait `From<Data1>` is not implemented for `Data2`
                                         // let data21: Data2 = &data1.into(); // compile error.  expected `Data2`, found `&_`
        let _data1: Data1 = (&data2).into(); // compile ok.  it seems like to strange,  the "()"
    }

    struct Data3 {
        name: String,
    }

    impl<T: AsRef<Data3>> From<T> for Data1 {
        fn from(value: T) -> Self {
            Self {
                name: value.as_ref().name.clone(),
            }
        }
    }

    impl AsRef<Data3> for Data3 {
        fn as_ref(&self) -> &Data3 {
            self
        }
    }

    #[test]
    fn test_ok_not_best() {
        let data3 = Data3 { name: "t".to_string() };
        let _data1 = Data1::from(&data3); // compile ok
        let _data1 = Data1::from(data3); // compile ok. but move the value data3 and call From<&>
                                         // let _data2 = Data1::from(data3);// compile error. the data3 moved and not impl Copy，so this is not best

        let data3 = Data3 { name: "t".to_string() };
        let _data1: Data1 = data3.into(); // compile error.  the trait `From<Data1>` is not implemented for `Data2`
                                          // let data3 = Data3{name:"t".to_string()};
                                          // let data1: Data1 = &data3.into(); // compile error.  expected `Data1`, found `&_`
        let data3 = Data3 { name: "t".to_string() };
        let _data1: Data1 = (&data3).into(); // compile error.  the trait `From<&Data1>` is not implemented for `Data2`
    }

    struct Data4 {
        name: String,
    }

    impl From<&Data4> for Data1 {
        fn from(value: &Data4) -> Self {
            Self { name: value.name.clone() }
        }
    }

    impl From<Data4> for Data1 {
        fn from(value: Data4) -> Self {
            Self { name: value.name }
        }
    }

    #[test]
    fn test_best_way() {
        let data4 = Data4 { name: "t".to_string() };
        let _data1 = Data1::from(&data4); // compile ok
        let _data1 = Data1::from(data4); // compile ok

        let data4 = Data4 { name: "t".to_string() };
        let _data1: Data1 = data4.into(); // compile ok
        let data4 = Data4 { name: "t".to_string() };
        // let data1: Data1 = &data4.into(); // compile error.  expected `Data1`, found `&_`
        let data4 = Data4 { name: "t".to_string() };
        let _data1: Data1 = (&data4).into(); // compile ok
    }
}
