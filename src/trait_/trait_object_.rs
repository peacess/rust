
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
    // {
    //     let s:std::raw::TraitObject;
    // }

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

