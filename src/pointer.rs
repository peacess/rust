use std::borrow::Cow;
use std::alloc::{Layout, alloc};
use std::ptr::null;
use std::ffi::{CString, CStr};

#[cfg(test)]
mod test {
    use std::fmt::{Pointer, Formatter};
    use std::{ptr, fmt};

    #[test]
    fn test_ptr() {
        let ref x : &bool = &false;
        // let v = Vec::new();
        #[derive(Debug)]
        struct Foo {
            a: i32
        }

        impl Pointer for Foo {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let ptr = self as *const Self;
                ptr.fmt(f)
            }
        }

        let mut f1 = Foo { a: 10 };
        unsafe {
            let f2 = ptr::read(&f1);
            println!("{:p}", f1);
            println!("{:p}", f2);
        }
    }
    #[test]
    fn test_drop_in_place(){
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
fn test_string(){
    //返回值分配一次内存
    fn str1(s: &str) -> String{
        //不要使用trim它只能去前后的空格
        let mut temp = String::with_capacity(s.len());
        for c in s.chars(){
            if c != ' '{
                temp.push(c);
            }
        }
        temp
    }
    //返回值分配一次内存，实现不如直接写for
    fn str2(s: &str) -> String{
        s.replace(" ","")
    }
    //返回值分配一次内存，实现不如直接写for
    fn str3(s: &str) -> String{
        //不要使用 skip_while 与 take_while，它们不是这个功能
        s.chars().filter(|c| *c != ' ').collect()
    }
    //参数如果是&str需要分配一次内存生成String; 如果是String但还要继续使用，会调用clone分配一次内存
    //返回值分配一次内存
    fn str4(s: String) -> String{
        let mut temp = String::with_capacity(s.len());
        for c in s.chars(){
            if c != ' '{
                temp.push(c);
            }
        }
        temp
    }
    //参数如果是&str需要分配一次内存生成String; 如果是String没有clone分配内存的问题
    //返回值分配一次内存
    fn str5(s: &String) -> String{
        let mut temp = String::with_capacity(s.len());
        for c in s.chars(){
            if c != ' '{
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
    fn str6(s:&str) ->Cow<str>{
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
        assert_eq!(ss,s1);
        let s2 = str2(&s);
        assert_eq!(ss,s2);
        let s3 = str3(&s);
        assert_eq!(ss,s3);
        let s4 = str4(s.clone());
        assert_eq!(ss,s4);
        let s5 = str5(&s);
        assert_eq!(ss,s5);
        println!("{}", s5);
    }

}

#[test]
fn test_raw_c_char(){
    use std::os::raw::c_char;
    use std::mem::ManuallyDrop;
    use std::mem::transmute;
    {
        let mut s = "".as_bytes().to_vec();
        s.push(0);
        let mut s = ManuallyDrop::new(s);
        let mut s = unsafe{Vec::from_raw_parts(s.as_mut_ptr() as *mut c_char, s.len(), s.capacity())};
        let mut raw_s = s.as_mut_ptr();
    }

    {
        let mut s = "5p";
        let temp = unsafe{transmute::<_,&[i8]>(s)};
        let mut s:Vec<c_char> = Vec::with_capacity(s.len() + 1);
        unsafe{s.set_len(s.capacity()-1);}
        s.copy_from_slice(temp);
        s.push(0);
        let mut raw_s = s.as_mut_ptr();
    }

    {
        let mut s = CString::new("").unwrap();
        let raw_s = s.into_raw();
        //free memory
        unsafe { CString::from_raw(raw_s); }
    }
}

#[test]
fn test_box(){
    struct Data{
        name: String,
    }
    impl Data{
        pub fn _init(&mut self){
            self.name = "test".to_owned();
        }
    }
    impl Default for Data{
        fn default() -> Self {
            let mut d = Data{name:String::default(),};
            d._init();
            d
        }
    }
    //这不是一个可靠的方法，只是为了说明思路，在实际的代码中不要这样使用
    let mut d = unsafe {
        let ptr:*mut Data = alloc(Layout::new::<Data>()) as _;
        (*ptr)._init();
        Box::from_raw(ptr)
    };
    assert_eq!("test", &d.name);
}

    #[test]
    fn test_trait_object() {
        use std::any::Any;
        use std::any::TypeId;
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
        let mut sub: &Sub = &MyStruct{};
        let mut parent:&Parent = &s;

        {//方法一，提供trait来转换
            let parent2: &Parent = sub.as_parent();
            println!("方法一，提供trait来转换: parent2.parent()");
            parent2.parent();
        }

        {//方法二，通过Any，实现转换
            let data = (sub.as_any()).downcast_ref::<MyStruct>().unwrap();
            let parents: &Parent = data;
            println!("方法二，通过Any，实现转换: parents.parent()");
            parents.parent();
        }

        {//方法三，通过unsafe代码实现 -- parent --> sub
            let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(parent) };
            let any: &Any = unsafe { &*data };
            let sub2: &Sub = unsafe { &*data };
            println!("方法三，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
            sub2.sub();
        }

        {//方法三，通过unsafe代码实现 -- parent --> sub
            let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(parent) };
            let sub2: &Sub = unsafe { &*data };
            println!("方法三，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
            sub2.sub();
        }

        {//方法四，通过unsafe代码实现 -- sub --> parent
            let (data,_) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(sub) };
            let parent2: &Parent = unsafe { &*data };
            println!("方法四，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
            parent2.parent();
        }

        {//方法五，通过unsafe代码实现 -- sub --> parent
            let parent2 = {
                let (data, _) = unsafe { transmute::<_, (*mut (), *mut ())>(sub) };
                let (_, v) = unsafe { transmute::<_, (*mut (), *mut ())>(unsafe{&*null::<MyStruct>()} as &Parent) };
                unsafe { transmute::<_, &Parent>((data, v)) }//直接组装一个trait object
            };
            println!("方法五，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
            parent2.parent();
        }
        {//方法六，通过unsafe代码实现 -- trait object --> any
            let parent2 = {
                let (_,v_any) = unsafe { transmute::<_, (*mut (), *mut ())>(&MyStruct{} as &Any) };
                let (data, _) = unsafe { transmute::<_, (*mut (), *mut ())>(sub) };
                let a =  unsafe { transmute::<_, &Any>((data,v_any)) };
                a.downcast_ref::<MyStruct>().unwrap() as &Parent
            };
            println!("方法六，通过unsafe代码实现 -- trait object --> any: parent2.parent()");
            parent2.parent();
        }
        println!("下面是错误的实现，可以运行，结果不正确");
        {
            let parent2 = unsafe { transmute::<_, &Parent>(sub) };
            println!("错误的实现，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
            parent2.parent();//out "sub..."，通过parent2的方法直接调用到Sub的方法了，因为parent2的vtable是直接指向parent的

            let sub2 = unsafe { transmute::<_, &Sub>(parent) };
            println!("错误的实现，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
            sub2.sub();//out "parent..."，通过sub2的方法直接调用到Parent的方法了，因为sub2的vtable是直接指向parent的

            //以上代码的运行结果,跟Parent和Sub中的方法定义的先后顺序有关，
        }

        //下面是TraitObject，与vtable的定义
        pub struct TraitObject {
            pub data: *mut (),
            pub vtable: *mut (),
        }
        // see: https://github.com/rust-lang/rust/blob/b63d7e2b1c4019e40051036bcb1fd5f254a8f6e2/src/librustc_codegen_llvm/meth.rs#L64-L115
        // see2: https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_ssa/src/meth.rs, 中的get_vtable
        // 这里的vtable只是按照它的内存布局来定义的，实际的实现是一个“Vec”
        struct Vtable {
            destructor: fn(*mut ()),
            size: usize,
            align: usize,
            method: [fn(*const ()) -> String;2],//这里是trait的方法数组，
            // “fn(*const ()) -> String”只是一例子，2也是
        }

    }