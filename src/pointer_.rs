#![allow(unused_variables)]

use std::alloc::{alloc, Layout};
use std::borrow::Cow;
use std::ffi::CString;
use std::ptr::null;
use std::cell::RefCell;

#[cfg(test)]
mod test {
    use std::{fmt, ptr};
    use std::fmt::{Formatter, Pointer};

    #[test]
    fn test_ptr() {
        // let v = Vec::new();
        #[derive(Debug)]
        struct Foo {
            a: i32,
        }

        impl Pointer for Foo {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let ptr = self as *const Self;
                ptr.fmt(f)
            }
        }

        let f1 = Foo { a: 10 };
        unsafe {
            let f2 = ptr::read(&f1);
            println!("{:p}", f1);
            println!("{:p}", f2);
        }
    }

    #[test]
    fn test_drop_in_place() {
        use std::ptr;
        use std::rc::Rc;

        let last = Rc::new(1);
        let weak = Rc::downgrade(&last);

        let mut v = vec![Rc::new(0), last];

        unsafe {
            // Get a raw pointer to the last element in `v`.
            let ptr = &mut v[1] as *mut _;
            // Shorten `v` to prevent the last item from being dropped. We do that first,
            // to prevent issues if the `drop_in_place` below panics.
            v.set_len(1);
            // Without a call `drop_in_place`, the last item would never be dropped,
            // and the memory it manages would be leaked.
            ptr::drop_in_place(ptr);
        }

        assert_eq!(v, &[0.into()]);

// Ensure that the last item was dropped.
        assert!(weak.upgrade().is_none());
    }
}

#[test]
fn test_string() {
    //返回值分配一次内存
    fn str1(s: &str) -> String {
        //不要使用trim它只能去前后的空格
        let mut temp = String::with_capacity(s.len());
        for c in s.chars() {
            if c != ' ' {
                temp.push(c);
            }
        }
        temp
    }
    //返回值分配一次内存，实现不如直接写for
    fn str2(s: &str) -> String {
        s.replace(" ", "")
    }
    //返回值分配一次内存，实现不如直接写for
    fn str3(s: &str) -> String {
        //不要使用 skip_while 与 take_while，它们不是这个功能
        s.chars().filter(|c| *c != ' ').collect()
    }
    //参数如果是&str需要分配一次内存生成String; 如果是String但还要继续使用，会调用clone分配一次内存
    //返回值分配一次内存
    fn str4(s: String) -> String {
        let mut temp = String::with_capacity(s.len());
        for c in s.chars() {
            if c != ' ' {
                temp.push(c);
            }
        }
        temp
    }
    //参数如果是&str需要分配一次内存生成String; 如果是String没有clone分配内存的问题
    //返回值分配一次内存
    fn str5(s: &String) -> String {
        let mut temp = String::with_capacity(s.len());
        for c in s.chars() {
            if c != ' ' {
                temp.push(c);
            }
        }
        temp
    }

    //以函数从入参上讲 &str最合理（注：String到&str没有新内存分配），不会分配不必要的内存，
    // 但是返回全都要分配一次内存。当字符串中不包含空格时，这一次的内存分配是多余的。
    // 入参数定下来了&str最好，那么返回值选什么类型，才能解决没有空格时的问题呢
    //返回为String一定会有内存分配，不行
    //返回为&str，没有空格时，没有内存分配，且可以工作，但是有空格时不行，因为重新分配内存后返回&str是编译器不允许的，生命周期不对
    //返回为Box<&str>，与上面一样的问题
    //经过查找发现Cow正是解决这个问题的，下面是它的实现
    //
    fn str6(s: &str) -> Cow<str> {
        if s.contains(' ') {
            let mut temp = String::with_capacity(s.len());
            for c in s.chars() {
                if c != ' ' {
                    temp.push(c);
                }
            }
            return Cow::Owned(temp);
        }
        return Cow::Borrowed(s);
    }

    {
        let mut s = "1 2";
        let s2 = str6(s);
        s = s2.as_ref();
        assert_eq!("12", s);
    }
    {
        let s = "1 2 ue".to_owned();
        let ss = "12ue".to_owned();
        let s1 = str1(&s);
        assert_eq!(ss, s1);
        let s2 = str2(&s);
        assert_eq!(ss, s2);
        let s3 = str3(&s);
        assert_eq!(ss, s3);
        let s4 = str4(s.clone());
        assert_eq!(ss, s4);
        let s5 = str5(&s);
        assert_eq!(ss, s5);
        println!("{}", s5);
    }
}

#[test]
fn test_raw_c_char() {
    use std::os::raw::c_char;
    use std::mem::ManuallyDrop;
    use std::mem::transmute;
    {
        let mut s = "".as_bytes().to_vec();
        s.push(0);
        let mut s = ManuallyDrop::new(s);
        let mut s = unsafe { Vec::from_raw_parts(s.as_mut_ptr() as *mut c_char, s.len(), s.capacity()) };
        let raw_s = s.as_mut_ptr();
    }

    {
        let s = "5p";
        let temp = unsafe { transmute::<_, &[i8]>(s) };
        let mut s: Vec<c_char> = Vec::with_capacity(s.len() + 1);
        unsafe { s.set_len(s.capacity() - 1); }
        s.copy_from_slice(temp);
        s.push(0);
        let raw_s = s.as_mut_ptr();
    }

    {
        let s = CString::new("").unwrap();
        let raw_s = s.into_raw();
        //free memory
        unsafe { CString::from_raw(raw_s); }
    }
}

#[test]
fn test_box() {
    struct Data {
        name: String,
    }
    impl Data {
        pub fn _init(&mut self) {
            self.name = "test".to_owned();
        }
    }
    impl Default for Data {
        fn default() -> Self {
            let mut d = Data { name: String::default() };
            d._init();
            d
        }
    }
    //这不是一个可靠的方法，只是为了说明思路，在实际的代码中不要这样使用
    let d = unsafe {
        let ptr: *mut Data = alloc(Layout::new::<Data>()) as _;
        (*ptr)._init();
        Box::from_raw(ptr)
    };
    assert_eq!("test", &d.name);
}

#[test]
fn test_trait_object() {
    use std::any::Any;
    use std::mem::transmute;

    trait Parent {
        fn parent(&self) {
            println!("parent...");
        }
    }
    trait AsParent {
        fn as_parent(&self) -> &dyn Parent;
    }
    // blanket implementation
    impl<T: Parent> AsParent for T {
        fn as_parent(&self) -> &dyn Parent {
            self
        }
    }

    trait Sub: AsParent {
        fn sub(&self) {
            println!("sub..");
        }
        fn as_any(&self) -> &dyn Any;
    }

    #[derive(Debug)]
    struct MyStruct;
    impl Sub for MyStruct {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl Parent for MyStruct {}
    let s = MyStruct;
    let sub: &dyn Sub = &MyStruct {};
    let parent: &dyn Parent = &s;

    {//方法一，提供trait来转换
        let parent2: &dyn Parent = sub.as_parent();
        println!("方法一，提供trait来转换: parent2.parent()");
        parent2.parent();
    }

    {//方法二，通过Any，实现转换
        let data = (sub.as_any()).downcast_ref::<MyStruct>().unwrap();
        let parents: &dyn Parent = data;
        println!("方法二，通过Any，实现转换: parents.parent()");
        parents.parent();
    }

    {//方法三，通过unsafe代码实现 -- parent --> sub
        let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(parent) };
        // let any: &dyn Any = unsafe { &*data };
        let sub2: &dyn Sub = unsafe { &*data };
        println!("方法三，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
        sub2.sub();
    }

    {//方法三，通过unsafe代码实现 -- parent --> sub
        let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(parent) };
        let sub2: &dyn Sub = unsafe { &*data };
        println!("方法三，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
        sub2.sub();
    }

    {//方法四，通过unsafe代码实现 -- sub --> parent
        let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(sub) };
        let parent2: &dyn Parent = unsafe { &*data };
        println!("方法四，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
        parent2.parent();
    }

    {//方法五，通过unsafe代码实现 -- sub --> parent
        let parent2 = {
            let (data, _) = unsafe { transmute::<_, (*mut (), *mut ())>(sub) };
            let (_, v) = unsafe { transmute::<_, (*mut (), *mut ())>(&*null::<MyStruct>() as &dyn Parent) };
            unsafe { transmute::<_, &dyn Parent>((data, v)) }//直接组装一个trait object
        };
        println!("方法五，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
        parent2.parent();
    }
    {//方法六，通过unsafe代码实现 -- trait object --> any
        let parent2 = {
            let (_, v_any) = unsafe { transmute::<_, (*mut (), *mut ())>(&MyStruct {} as &dyn Any) };
            let (data, _) = unsafe { transmute::<_, (*mut (), *mut ())>(sub) };
            let a = unsafe { transmute::<_, &dyn Any>((data, v_any)) };
            a.downcast_ref::<MyStruct>().unwrap() as &dyn Parent
        };
        println!("方法六，通过unsafe代码实现 -- trait object --> any: parent2.parent()");
        parent2.parent();
    }
    println!("下面是错误的实现，可以运行，结果不正确");
    {
        let parent2 = unsafe { transmute::<_, &dyn Parent>(sub) };
        println!("错误的实现，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
        parent2.parent();//out "sub..."，通过parent2的方法直接调用到Sub的方法了，因为parent2的vtable是直接指向parent的

        let sub2 = unsafe { transmute::<_, &dyn Sub>(parent) };
        println!("错误的实现，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
        sub2.sub();//out "parent..."，通过sub2的方法直接调用到Parent的方法了，因为sub2的vtable是直接指向parent的

        //以上代码的运行结果,跟Parent和Sub中的方法定义的先后顺序有关，
    }

    //下面是TraitObject，与vtable的定义
    #[allow(dead_code)]
    pub struct TraitObject {
        pub data: *mut (),
        pub vtable: *mut (),
    }
    // see: https://github.com/rust-lang/rust/blob/b63d7e2b1c4019e40051036bcb1fd5f254a8f6e2/src/librustc_codegen_llvm/meth.rs#L64-L115
    // see2: https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_ssa/src/meth.rs, 中的get_vtable
    // 这里的vtable只是按照它的内存布局来定义的，实际的实现是一个“Vec”
    #[allow(dead_code)]
    struct Vtable {
        destructor: fn(*mut ()),
        size: usize,
        align: usize,
        method: [fn(*const ()) -> String; 2],//这里是trait的方法数组，
        // “fn(*const ()) -> String”只是一例子，2也是
    }
}

#[test]
fn test_ref() {
    let data = 10;
    let t1 = &data;
    let t2 = &&data;
    let t3 = &&&data;
    let t4 = &&&&data;

    assert_eq!(t1, *t2);//值相等
    {
        assert!(t1 == *t2);//same as assert_eq, PartialEq
        let p1 = t1 as *const _;
        let p2 = *t2 as *const _;
        assert!(p1 == p2); //raw pointer并没有实现 PartialEq，而是地址是否相等

        let p1 = &p1 as *const *const _;
        let p2 = &p2 as *const *const _;
        assert!(p1 != p2);//raw pointer并没有实现 PartialEq，而是地址是否相等
    }
    assert!(std::ptr::eq(t1, *t2));//指针地址相等
    assert_eq!(t2, *t3);//值相等
    assert!(!std::ptr::eq(t2, *t3));//指针地址不相等

    assert_eq!(t3, *t4);//值相等
    assert!(!std::ptr::eq(t3, *t4));//指针地址不相等

    assert_eq!(t1, **t3);//值相等
    assert!(std::ptr::eq(t1, **t3));//指针地址相等

    assert_eq!(t1, ***t4);//值相等
    assert!(std::ptr::eq(t1, ***t4));//指针地址相等
}

/// 总结
/// 1. 引用是一种特殊的指针类型
/// 2. 引用在赋值或传参数时（传参数相当给函数参数赋值），会产生一个新的复本对象，这个新对象的值相同（这个值都是data变量的内存地址）
/// 3. 因为产生副本所以&&的值不相同，&&的值是新复本对象的地址
/// 4. 当在for中时，&&的值相同，原因每次循环生成的新复本对象的地址相同，这里是stack方式的内存，所以二次产生的副本对象地址相同。
/// 5. 引用在相等比较时，是去掉所有的引用，比较的最终的值是否等
/// 6. raw pointer在比较时，是直接比较当前对象的值（也就是指针）是否相待， 不会使用*解引后比较。
#[test]
fn test_ref_parameters() {
    let data = 1;
    let ref_data = &data;
    let ref2_data = &data;
    let ref3_data = ref_data;
    let d_ref_data = &&data;
    let d2_ref_data = &&data;

    println!("//二次&data指针相等");
    println!("&data: {:p}", &data);
    println!("&data: {:p}", &data);
    println!("//二次for的&data指针相等");
    for _ in 0..2 {
        println!("&data in for: {:p}", &data);
    }
    println!("//二次&&data指针不相等");
    println!("&&data: {:p}", &&data);
    println!("&&data: {:p}", &&data);
    println!("//二次for的&&data指针相等");
    for _ in 0..2 {
        println!("&&data in for: {:p}", &&data);
    }
    println!("//二次赋值给变量的&&data指针不相等");
    println!("&&data: {:p}", d_ref_data);
    println!("&&data: {:p}", d2_ref_data);

    println!("ref_data: {:p}", ref_data);
    println!("ref2_data: {:p}", ref2_data);
    println!("ref3_data: {:p}", ref3_data);
    println!("&ref_data: {:p}", &ref_data);
    println!("&ref2_data: {:p}", &ref2_data);
    println!("&ref3_data: {:p}", &ref3_data);

    assert_eq!(&data, ref_data);
    assert_eq!(&data, ref2_data);
    assert_eq!(&data, ref3_data);
    assert_eq!(&data, &data);

    assert_eq!(&&data, &ref_data);
    assert_eq!(&&data, &ref2_data);
    assert_eq!(&&data, &ref3_data);
    assert_eq!(&&data, &&data);

    fn f(ref_f: &i32, raw: *const i32) {
        println!("ref_f in f : {:p}:{}", ref_f, ref_f);
        println!("&ref_f in f : {:p}:{}", &ref_f, &ref_f);
        let u2 = ref_f as *const i32;
        assert_eq!(u2, raw);
    }

    let raw_data: *const i32 = &data;
    f(&data, raw_data);
}

#[test]
fn test_ref_self() {}

#[test]
fn test_eq() {
    let s = 1;
    let s2 = 1;
    {//值相同，但指针不同
        assert!(s == s2);
        assert!(&s == &s2);

        let ref_s = &s;
        let ref_s2 = &s2;

        let raw_s = ref_s as *const i32;
        let raw_s2 = ref_s2 as *const i32;

        assert!(ref_s == ref_s2);

        assert_ne!(raw_s, raw_s2);
        assert!(raw_s != raw_s2);
        assert!(!std::ptr::eq(ref_s, ref_s2));
        assert!(!std::ptr::eq(raw_s, raw_s2));
    }
    {//同一变量的引用
        let ref_s = &s;
        let ref_s2 = &s;
        assert_eq!(ref_s, ref_s2);
        assert_eq!(&ref_s, &ref_s2);

        let d_ref_s = &&s;
        let d_ref_s2 = &&s;
        assert_eq!(d_ref_s, d_ref_s2);

        assert_eq!(d_ref_s, &ref_s);

        let raw_s = ref_s as *const i32;
        let raw_s2 = ref_s2 as *const i32;
        assert_eq!(raw_s, raw_s2);

        let mut t = std::ptr::null::<i32>();
        let d_raw_s: *mut *const i32 = &mut t;
        let mut t = std::ptr::null::<i32>();
        let d_raw_s2: *mut *const i32 = &mut t;
        unsafe {
            *d_raw_s = raw_s;
            *d_raw_s2 = raw_s2;
        }
        assert_ne!(d_raw_s, d_raw_s2);
    }
}

#[test]
fn test_double_ref_raw() {
    let data = 9;
    let ref_data = &data;

    let raw_data = ref_data as *const i32;
    // let d_raw_data = &ref_data as *const *const i32; //error[E0606]: casting `&&i32` as `*const *const i32` is invalid
    let d_ref_data = &&data;
    // let d_raw_data = d_ref_data as *const *const i32; //error[E0606]: casting `&&i32` as `*const *const i32` is invalid
    // why “casting `&&i32` as `*const *const i32` is invalid”
    // [see](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)

    //error sample, never use it
    fn err_d_raw() -> *mut *mut i32 {
        let t: *mut *mut i32 = &mut std::ptr::null_mut();
        t
    }

    fn ok1_d_raw<T>() -> *mut *mut T {
        let t = Box::new(std::ptr::null_mut());
        Box::into_raw(t)
    }
    fn ok2_d_raw<T>() -> *mut *mut T {//可以正确工作，但为了统一free内存，不建议这种方式
        let layout = std::alloc::Layout::new::<*mut *mut T>();
        //注意分配内存时，一定注意是否需要 zeroed.
        let t = unsafe { std::alloc::alloc_zeroed(layout) } as *mut *mut T;
        t
    }

    fn free_d_raw<T>(d: &mut *mut *mut T) {
        if *d != std::ptr::null_mut() {
            unsafe {
                let f = &mut **d;//是两个*号，第一个对应的是 &mut
                if *f != std::ptr::null_mut() {
                    let _ = Box::from_raw(*f);
                    *f = std::ptr::null_mut();
                }
                let _ = Box::from_raw(*d);
                *d = std::ptr::null_mut();
            }
        }
    }
    let d1 = err_d_raw();//不要释放这个内存，它是stack，释放会产生未知错误
    let mut d2 = ok1_d_raw::<i32>();
    free_d_raw(&mut d2);
    let mut d3 = ok2_d_raw::<i32>(); //由于使用的alloc直接分配的内存，最好使用dealloc来free内存，配对使用
    free_d_raw(&mut d3);

    let mut d4 = ok1_d_raw::<i32>();
    unsafe { *d4 = Box::into_raw(Box::new(10)); }
    free_d_raw(&mut d4);

    assert_ne!(d1, std::ptr::null_mut());
    assert_eq!(d2, std::ptr::null_mut());
    assert_eq!(d3, std::ptr::null_mut());
    assert_eq!(d4, std::ptr::null_mut());

    println!("err_d_raw d1: {:p}\nok1_d_raw d2: {:p}\nok2_d_raw d3: {:p}", d1, d2, d3);
}


#[test]
fn test_auto_copy() {
    #[derive(Debug)]
    struct D {
        a: i32,
    }

    let d = D{a:10};
    let d2 = d;
    // println!("{:?}", d);
}

#[test]
fn test_ref_box(){
    let t = RefCell::new(Box::new(Vec::new()));
    t.borrow_mut().push(1);

}
