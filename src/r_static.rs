#[cfg(test)]
mod test {
    #![allow(unused_variables)]

    use std::cell::{Cell, UnsafeCell};
    use std::sync::Once;

    use lazy_static::lazy_static;
    use static_init::dynamic;

    #[allow(dead_code)]
    static G_VALUE: i32 = 3;
    #[allow(dead_code)]
    static mut M_VALUE: u32 = 0;

    pub fn fn_static() -> i32 {
        static STATIC_FIELD: i32 = 10;//在第一次调用时分配，多次调用，也不会初始化多次
        return STATIC_FIELD;
    }

    pub struct GlobalData {}

    impl GlobalData {
        pub fn new() -> GlobalData {
            GlobalData {}
        }
    }

    pub fn global_data_new() -> GlobalData {
        GlobalData {}
    }

    // static G_STRING1:String = String::new();
// static G_STRING2:String = "".to_owned();
    #[allow(dead_code)]
    static G_STRING: &str = "same string";
// static G_DATA:GlobalData = GlobalData{name:"".to_owned()};

    pub fn fn_g_string2() -> &'static String {
        static ONE: Once = Once::new();
        static mut DATA: Cell<Option<String>> = Cell::new(None);
        ONE.call_once(|| {
            unsafe { DATA.set(Some("string fn_g_string2".to_owned())); }
        });
        unsafe { DATA.get_mut() }.as_ref().expect("static is not init")
    }

    pub fn fn_g_string3() -> &'static String {
        static ONE: Once = Once::new();
        static mut DATA: UnsafeCell<Option<String>> = UnsafeCell::new(None);
        ONE.call_once(|| {
            unsafe { *DATA.get() = Some("string fn_g_string3".to_owned()) };
        });
        unsafe { &*DATA.get() }.as_ref().expect("static is not init")
    }

    struct StaticData<T> {
        _one: Once,
        _data: UnsafeCell<Option<T>>,
    }

    #[allow(dead_code)]
    impl<T> StaticData<T> {
        pub const fn new() -> Self {
            StaticData {
                _one: Once::new(),
                _data: UnsafeCell::new(None),
            }
        }

        pub fn get<F: FnOnce() -> T>(&'static self, builder: F) -> &'static T {
            self._one.call_once(|| {
                unsafe { *self._data.get() = Some(builder()); }
            });
            unsafe { &*self._data.get() }.as_ref().expect("static not init")
        }
    }

    unsafe impl<T: Sync> Sync for StaticData<T> {}

    unsafe impl<T: Sync> Send for StaticData<T> {}

    #[allow(dead_code)]
    fn demo_static_data() -> &'static String {
        static S_DATA: StaticData<String> = StaticData::new();
        S_DATA.get(|| "some string".to_owned())
    }

    lazy_static! {
    static ref G_STRING1:String = String::new();
}

// static CELL: OnceCell<i32> = OnceCell::INIT;


// struct G_STRING1 {
//     __private_field: (),
// }
// #[doc(hidden)]
// static G_STRING1: G_STRING1 = G_STRING1 {
//     __private_field: (),
// };
// impl ::lazy_static::__Deref for G_STRING1 {
//     type Target = String;
//     fn deref(&self) -> &String {
//         #[inline(always)]
//         fn __static_ref_initialize() -> String {
//             {
//                 String::new()
//             }
//         }
//         #[inline(always)]
//         fn __stability() -> &'static String {
//             static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
//             LAZY.get(__static_ref_initialize)
//         }
//         __stability()
//     }
// }
// impl ::lazy_static::LazyStatic for G_STRING1 {
//     fn initialize(lazy: &Self) {
//         let _ = &**lazy;
//     }
// }

    struct Data {
        pub v: i32,
    }

    impl Drop for Data {
        fn drop(&mut self) {
            println!("drop ");
        }
    }

    #[dynamic(lazy, drop)]
    static mut GD: Data = Data { v: 10 };

    #[test]
    fn test_static_init() {
        GD.write().v = 6;
        // unsafe { GD.v = 6; }
    }

    #[test]
    fn test_static_lifetime() {
        fn f(a: &'static i32) {
            println!("a's type:{},address: {:p}", std::any::type_name::<&'static i32>(), a);
        }
        fn f2<T: 'static>(a: &T) {
            println!("a's type:{},address: {:p}", std::any::type_name::<&T>(), a);
        }
        fn f3<T: 'static>(a: T) {
            println!("a's type:{},address: {:p}", std::any::type_name::<T>(), &a);
        }
        {
            println!("First part ====================================================");
            let data = 5;
            // f(&data);//`data` does not live long enough
            const DATA7: i32 = 7;
            print!("{:<30}", "f(&DATA7);");
            f(&DATA7);
            print!("{:<30}", "f(&&DATA7);");
            f(&&DATA7);
            static DATA2: i32 = 8;
            print!("{:<30}", "f(&DATA2);");
            f(&DATA2);
            print!("{:<30}", "f(&&DATA2);");
            f(&&DATA2);//自动解引用，实际输入参数为 &DATA2
        }
        {
            println!("Second part ====================================================");
            let data = 5;
            print!("{:<30}", "f2(&data);");
            f2(&data);//虽然data是局部变量，但是它仍然满足'static要求。
            // f2(&&data);//编译不通过“`data` does not live long enough”。data满足'static要求，但是&data不满足'static
            let data = 6;
            f2(&data);
            let data = vec![1, 2];
            f2(&data);
            static DATA2: i32 = 8;
            println!("data address:{:p},DATA2 address:{:p}", &data, &DATA2);
            print!("{:<30}", "f2(&DATA2);");
            f2(&DATA2);//f2::<i32>(&DATA2);
            print!("{:<30}", "f2(&&DATA2);");
            f2(&&DATA2);//f2::<&i32>(&&DATA2);
            // f2(&&&DATA2);//f2::<&&i32>(&&DATA2); 编译不通过“temporary value is freed at the end of this statement”
            println!("call f2::<i32>");
            print!("{:<30}", "f2::<i32>(&DATA2);");
            f2::<i32>(&DATA2);
            print!("{:<30}", "f2::<i32>(&&DATA2);");
            f2::<i32>(&&DATA2);//自动解引用，实际输入参数为 &DATA2
            print!("{:<30}", "f2::<i32>(*&&&&DATA2);");
            f2::<i32>(*&&&DATA2);//自动解引用，实际输入参数为 &DATA2
            //上面三个函数调用，参数的值都是相等的，都是&DATA2，也就是data2的地址
            println!("call f2::<&i32>");
            print!("{:<30}", "f2::<&i32>(&&DATA2);");
            f2::<&i32>(&&DATA2);//实际输入参数为 &&DATA2
            print!("{:<30}", "f2::<&i32>(&&&DATA2);");
            f2::<&i32>(&&&DATA2);//自动解引用，实际输入参数为 &&DATA2
            print!("{:<30}", "f2::<&i32>(**&&&&DATA2);");
            f2::<&i32>(**&&&&DATA2);//实际输入参数为 &&DATA2
            //上面三个函数调用，但是值不相等，但deref值（*&&DATA2 == &DATA2 ）是相等的。第二次引用是一个临时变化的地址
            // f2::<&&i32>(&&&DATA2);//编译不通过“temporary value is freed at the end of this statement”
        }
        {
            println!("Third part ====================================================");
            println!("call f3");
            let data = 5;
            print!("{:<30}", "f3(data);");
            f3(data);//f3::<i32> ---> f3(a: 'static i32)
        // f3(&data);//编译不通过“`data` does not live long enough”
        static DATA2: i32 = 8;
            println!("data address:{:p},DATA2 address:{:p}", &data, &DATA2);
            print!("{:<30}", "f3(&DATA2);");
            f3(&DATA2);
            print!("{:<30}", "f3(DATA2);");
            f3(DATA2);
            // f3(&&DATA2);//error "temporary value dropped while borrowed"
            println!("call f3::<i32>");
            print!("{:<30}", "f3::<i32>(*&DATA2);");
            f3::<i32>(*&DATA2);
            print!("{:<30}", "f3::<i32>(**&&DATA2);");
            f3::<i32>(**&&DATA2);
            println!("call f3::<&i32>");
            print!("{:<30}", "f3::<&i32>(&DATA2);");
            f3::<&i32>(&DATA2);
            print!("{:<30}", "f3::<&i32>(&&DATA2);");
            f3::<&i32>(&&DATA2);
            //问题： 当调用f3时，只要泛型参数相同（由于泛型的单态化，泛型参数相同，说明是同一个函数），输出的地址也相同，如果f3(data)与f3(DATA2)输出的地址相等。为什么？
            //原因之一是f3中输出的是参数值的内存地址，而不是data或data2的地址。为什么同一个函数，参数地址相同，这个还没有搞清楚是什么原因
        }
    }
}