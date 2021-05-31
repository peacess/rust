use lazy_static::lazy_static;
use std::cell::{RefCell, Cell, UnsafeCell};
use std::sync::Once;
use std::ops::{Deref, DerefMut};
use once_cell::sync::OnceCell;

static G_VALUE: i32 = 3;
static mut M_VALUE:u32 = 0;

pub fn fn_static() -> i32{
    static STATIC_FIELD: i32 = 10;//在第一次调用时分配，多次调用，也不会初始化多次
    return STATIC_FIELD;
}

pub struct GlobalData{
    name: String,
}

impl GlobalData{
    pub fn new()-> GlobalData{
        GlobalData{name:"".to_owned()}
    }
}

pub fn GlobalData_new() -> GlobalData{
    GlobalData{name:"".to_owned()}
}

// static G_STRING1:String = String::new();
// static G_STRING2:String = "".to_owned();
static G_STRING:&str = "same string";
// static G_DATA:GlobalData = GlobalData{name:"".to_owned()};

pub fn FN_G_STRING2() -> &'static String {
    static one:Once = Once::new();
    static mut data: Cell<Option<String>> = Cell::new(None);
    one.call_once(|| {
        unsafe { data.set(Some("string FN_G_STRING2".to_owned())); }
    });
    unsafe { data.get_mut()}.as_ref().expect("static is not init")
}
pub fn FN_G_STRING3() -> &'static String {
    static one:Once = Once::new();
    static mut data: UnsafeCell<Option<String>> = UnsafeCell::new(None);
    one.call_once(|| {
        unsafe { *data.get() = Some("string FN_G_STRING3".to_owned())};
    });
    unsafe { &*data.get()}.as_ref().expect("static is not init")
}

struct StaticData<T>{
    one: Once,
    data: UnsafeCell<Option<T>>,
}

impl<T> StaticData<T>{
    pub const fn new() -> Self {
        StaticData {
            one: Once::new(),
            data: UnsafeCell::new(None),
        }
    }

    pub fn get<F: FnOnce() -> T>(&'static self, builder: F) -> &'static T {
        self.one.call_once(||{
            unsafe { *self.data.get() = Some(builder()); }
        });
        unsafe { &*self.data.get()}.as_ref() .expect("static not init")
    }
}

unsafe impl<T: Sync> Sync for StaticData<T> {}
unsafe impl<T: Sync> Send for StaticData<T> {}

fn demo_static_data() ->&'static String{
    static s_data: StaticData<String> = StaticData::new();
    s_data.get(||"some string".to_owned())
}

lazy_static!{
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