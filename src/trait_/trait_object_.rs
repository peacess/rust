#[cfg(test)]
mod test {
    #![cfg_attr(feature = "unstable", feature(ptr_metadata))]

    use std::ptr::null;
    #[cfg(feature = "unstable")]
    use std::ptr::{metadata, Pointee};

    const A: &str = "sdf";
    static B: &str = "sd";

    /// 在sub trait object之间进行转换是可以的，但需要使用非安全代码，其中metadata(方法七，方法八)需要nightly
    /// 通过下面的测试可以有八种方法可以实现，建议使用方法为，定义一个转换的trait或单独在实现类中增加方法来进行完全转换，也就是方法一
    ///
    /// 从unsafe实现中可以得出如下结果：
    /// 1. trait object是一个fat 指针，包含struct的地址，及一个trait 的metadata（主要内容为vtable）
    /// 2. any的typeid是在编译确定的，
    /// 3。trait object中没有存放struct的类型信息（如，字段等），所以不能通过"反射"来取到字段
    /// 4。不能通过trait object来取到struct的方法，只能取到trait的方法？（这一条还不是很确定，有进一步验证）
    ///
    ///
    #[test]
    fn test() {
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

        {
            //方法一，提供trait来转换
            let parent2: &dyn Parent = sub.as_parent();
            println!("方法一，提供trait来转换: parent2.parent()");
            parent2.parent();
        }

        {
            //方法二，通过Any，实现转换
            let data = (sub.as_any()).downcast_ref::<MyStruct>().unwrap();
            let parents: &dyn Parent = data;
            println!("方法二，通过Any，实现转换: parents.parent()");
            parents.parent();
        }

        {
            //方法三，通过unsafe代码实现 -- parent --> sub
            let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(parent) };
            // let any: &dyn Any = unsafe { &*data };
            let sub2: &dyn Sub = unsafe { &*data };
            println!("方法三，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
            sub2.sub();
        }

        {
            //方法三，通过unsafe代码实现 -- parent --> sub
            let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(parent) };
            let sub2: &dyn Sub = unsafe { &*data };
            println!("方法三，通过unsafe代码实现 -- parent --> sub: sub2.sub()");
            sub2.sub();
        }

        {
            //方法四，通过unsafe代码实现 -- sub --> parent
            let (data, _) = unsafe { transmute::<_, (*mut MyStruct, *mut ())>(sub) };
            let parent2: &dyn Parent = unsafe { &*data };
            println!("方法四，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
            parent2.parent();
        }

        {
            //方法五，通过unsafe代码实现 -- sub --> parent
            let parent2 = {
                let (data, _) = unsafe { transmute::<_, (*mut (), *mut ())>(sub) };
                #[allow(deref_nullptr)]
                let (_, v) = unsafe { transmute::<_, (*mut (), *mut ())>(&*null::<MyStruct>() as &dyn Parent) };
                unsafe { transmute::<_, &dyn Parent>((data, v)) } //直接组装一个trait object
            };
            println!("方法五，通过unsafe代码实现 -- sub --> parent: parent2.parent()");
            parent2.parent();
        }
        {
            //方法六，通过unsafe代码实现 -- trait object --> any
            let parent2 = {
                let (_, v_any) = unsafe { transmute::<_, (*mut (), *mut ())>(&MyStruct {} as &dyn Any) };
                let (data, _) = unsafe { transmute::<_, (*mut (), *mut ())>(sub) };
                let a = unsafe { transmute::<_, &dyn Any>((data, v_any)) };
                a.downcast_ref::<MyStruct>().unwrap() as &dyn Parent
            };
            println!("方法六，通过unsafe代码实现 -- trait object --> any: parent2.parent()");
            parent2.parent();
        }
        #[cfg(feature = "unstable")]
        {
            unsafe {
                //方法七，通过 metadata来， parent --> sub，此方法需要nightly版
                let (data, parent_meta) = (parent as *const Parent).to_raw_parts();
                let sub2 = ptr::from_raw_parts::<Sub>(data, (ptr::null::<MyStruct>() as *const Sub).to_raw_parts().1);
                println!("方法七，通过 metadata来， parent --> sub，此方法需要nightly版");
                (*sub2).sub();
            }
            unsafe {
                //方法八，通过 metadata来， sub --> parent，此方法需要nightly版
                let (data, sub_meta) = (sub as *const Sub).to_raw_parts();
                let parent2 = ptr::from_raw_parts::<Parent>(data, (ptr::null::<MyStruct>() as *const Parent).to_raw_parts().1);
                println!("方法八，通过 metadata来， sub --> parent，此方法需要nightly版");
                (*parent2).parent();
            }
        }

        //下面是TraitObject，与vtable的定义
        //这个对象在 1.53被删除， 但内存关系还是一样的
        {
            #[allow(dead_code)] //为了去掉编译的waring
            pub struct TraitObject {
                pub data: *mut (),
                pub vtable: *mut (),
            }
            // see: https://github.com/rust-lang/rust/blob/b63d7e2b1c4019e40051036bcb1fd5f254a8f6e2/src/librustc_codegen_llvm/meth.rs#L64-L115
            // see2: https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_ssa/src/meth.rs, 中的get_vtable
            // 这里的vtable只是按照它的内存布局来定义的，实际的实现是一个“Vec”
            #[allow(dead_code)] //为了去掉编译的waring
            struct Vtable {
                destructor: fn(*mut ()),
                size: usize,
                align: usize,
                method: [fn(*const ()) -> String; 2], //这里是trait的方法数组，
                                                      // “fn(*const ()) -> String”只是一例子，2也是
            }
        }

        //新版的定义如下：
        #[cfg(feature = "unstable")]
        {
            #[allow(dead_code)] //为了去掉编译的waring
            pub(crate) struct PtrComponents<T: ?Sized> {
                pub(crate) data_address: *const (),
                pub(crate) metadata: <T as Pointee>::Metadata, //这里就是vtable
            }

            #[allow(dead_code)] //为了去掉编译的waring
            struct VTable {
                drop_in_place: fn(*mut ()),
                size_of: usize,
                align_of: usize,
            }
        }
    }
}
