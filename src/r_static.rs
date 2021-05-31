use std::cell::{Cell, RefCell, UnsafeCell};
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::sync::Once;

use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

static G_VALUE: i32 = 3;
static mut M_VALUE: u32 = 0;

pub fn fn_static() -> i32 {
    static STATIC_FIELD: i32 = 10;//在第一次调用时分配，多次调用，也不会初始化多次
    return STATIC_FIELD;
}

pub struct GlobalData {
    name: String,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        GlobalData { name: "".to_owned() }
    }
}

pub fn GlobalData_new() -> GlobalData {
    GlobalData { name: "".to_owned() }
}

// static G_STRING1:String = String::new();
// static G_STRING2:String = "".to_owned();
static G_STRING: &str = "same string";
// static G_DATA:GlobalData = GlobalData{name:"".to_owned()};

pub fn FN_G_STRING2() -> &'static String {
    static one: Once = Once::new();
    static mut data: Cell<Option<String>> = Cell::new(None);
    one.call_once(|| {
        unsafe { data.set(Some("string FN_G_STRING2".to_owned())); }
    });
    unsafe { data.get_mut() }.as_ref().expect("static is not init")
}

pub fn FN_G_STRING3() -> &'static String {
    static one: Once = Once::new();
    static mut data: UnsafeCell<Option<String>> = UnsafeCell::new(None);
    one.call_once(|| {
        unsafe { *data.get() = Some("string FN_G_STRING3".to_owned()) };
    });
    unsafe { &*data.get() }.as_ref().expect("static is not init")
}

struct StaticData<T> {
    one: Once,
    data: UnsafeCell<Option<T>>,
}

impl<T> StaticData<T> {
    pub const fn new() -> Self {
        StaticData {
            one: Once::new(),
            data: UnsafeCell::new(None),
        }
    }

    pub fn get<F: FnOnce() -> T>(&'static self, builder: F) -> &'static T {
        self.one.call_once(|| {
            unsafe { *self.data.get() = Some(builder()); }
        });
        unsafe { &*self.data.get() }.as_ref().expect("static not init")
    }
}

unsafe impl<T: Sync> Sync for StaticData<T> {}

unsafe impl<T: Sync> Send for StaticData<T> {}

fn demo_static_data() -> &'static String {
    static s_data: StaticData<String> = StaticData::new();
    s_data.get(|| "some string".to_owned())
}

lazy_static! {
    static ref G_STRING1:String = {String::new()};
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
#[test]
fn test_static_lifetime() {
    {
        fn f(a: &'static i32) {}
        let data = 5;
        // f(&data);//`data` does not live long enough
        static data2: i32 = 8;
        f(&data2);
        f(&&data2);
    }
    fn f2<T: 'static + Debug>(a: &T) {
        println!("a's type:{},address: {:p}", std::any::type_name::<&T>(), a);
    }
    fn f3<T: 'static + Sized>(a: T) {
        println!("a's type:{},address: {:p}", std::any::type_name::<T>(), &a);
    }
    {//第一部分
        let data = 5;
        f2(&data);//虽然data是局部变量，但是它仍然满足'static要求。
        // f2(&&data);//编译不通过“`data` does not live long enough”。data满足'static要求，但是&data不满足'static


        static data2: i32 = 8;
        println!("static value,data p:{:p},static data p:{:p}", &data, &data2);
        f2(&data2);//f2::<i32>(&data2);
        f2(&&data2);//f2::<&i32>(&&data2);
        // f2(&&&data2);//f2::<&&i32>(&&data2); 编译不通过“temporary value is freed at the end of this statement”
        println!("call f2::<i32>");
        f2::<i32>(&data2);
        f2::<i32>(&&data2);//自动解引用，实际输入参数为 &data2，参数的地址是相等的
        f2::<i32>(&&&data2);//自动解引用，实际输入参数为 &data2
        f2::<i32>(&&&&data2);//自动解引用，实际输入参数为 &data2
        f2::<i32>(*&&&&data2);//自动解引用，实际输入参数为 &data2
        f2::<i32>(**&&&&data2);//自动解引用，实际输入参数为 &data2
        //上面六个函数调用，参数的值都是相等的，都是&data2，也就是data2的地址
        println!("call f2::<i32>");
        f2::<&i32>(&&data2);//实际输入参数为 &&data2
        f2::<&i32>(&&&data2);//自动解引用，实际输入参数为 &&data2
        f2::<&i32>(&&&&data2);//自动解引用，实际输入参数为 &&data2
        f2::<&i32>(*&&&data2);//自动解引用，实际输入参数为 &&data2
        f2::<&i32>(**&&&&data2);//实际输入参数为 &&data2
        //上面五个函数调用，但是值不相等，但deref值（*&&data2 == &data2 ）是相等的。第二次引用是一个临时变化的地址
        // f2::<&&i32>(&&&data2);//编译不通过“temporary value is freed at the end of this statement”
    }
    {//第二部分
        println!("\ncall f3");
        let data = 5;
        f3(data);//f3::<i32> ---> f3(a: 'static i32)
        // f3(&data);//编译不通过“`data` does not live long enough”

        static data2: i32 = 8;
        println!("static value,data p:{:p},static data p:{:p}", &data, &data2);
        f3(&data2);
        f3(data2);
        // f3(&&data2);//error "temporary value dropped while borrowed"
        println!("call f3::<i32>");
        f3::<i32>(*&data2);
        f3::<i32>(**&&data2);
        println!("call f3::<&i32>");
        f3::<&i32>(&data2);
        f3::<&i32>(&&data2);
        //问题： 当调用f3时，只要a的类型相同，输出的地址也相同，如果f3(data)与f3(data2)输出的地址相等。为什么？
    }
}