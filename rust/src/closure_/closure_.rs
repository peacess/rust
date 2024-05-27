#[test]
fn test_closure() {
    /// 总结
    /// 1. FnOnce只能调用一次
    /// 2. 加move的closure可以是任意三种
    /// 3. 加move的

    fn fn_once<F>(func: F)
    where
        F: FnOnce(),
    {
        func(); //只能调用一次，&func也不行
    }
    {
        let mut a = "fn_once".to_owned();
        let f = || {
            // a = capture(a);
            println!("{}", a)
        };
        let f2 = f;
        fn_once(f); //f 自动实现了 Copy,所以可以再一次使用变量f
        fn_once(f2);
    }
    {
        fn fn_once<F>(func: F)
        where
            F: FnOnce(),
        {
            func()
        }
        let a = "fn_once move".to_owned();
        let f = move || println!("{}", a);
        f();
        (&f)();
        fn_once(&f); //可以多次调用
        fn_once(f);
        //fn_once(f); //error[E0382]: use of moved value: `f`。有move时不会实现 Copy
    }
    fn fn_<F>(func: F)
    where
        F: Fn(),
    {
        func();
    }
    {
        let a = "fn".to_owned();
        let f = || println!("{}", a);
        let f2 = f;
        fn_(f); //f 自动实现了 Copy,所以可以再一次使用变量f
        fn_(f2);
    }
    {
        let a = "fn move".to_owned();
        let f = move || println!("{}", a);
        fn_(&f);
        fn_(f);
    }

    fn fn_mut<F>(mut func: F)
    where
        F: FnMut(),
    {
        func();
    }
    {
        let mut a = "FnMut".to_owned();
        let mut f = || {
            a.push_str("X");
            println!("{}", a);
        };
        f();
        (&mut f)();
        fn_mut(&mut f);
        fn_mut(f);
    }
    {
        let mut a = "FnMut move".to_owned();
        let f = move || {
            a.push_str("X");
            println!("{}", a);
        };
        fn_mut(f);
        // fn_mut(f);
    }
    // { nightly work
    //     struct Data {}
    //
    //     impl FnOnce<()> for Data {
    //         type Output = ();
    //         extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
    //             ()
    //         }
    //     }
    //
    //     impl FnMut<()> for Data {
    //         extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
    //             ()
    //         }
    //     }
    //
    //     impl Fn<()> for Data {
    //         extern "rust-call" fn call(&self, args: ()) -> Self::Output {
    //             ()
    //         }
    //     }
    //     let d = Data {};
    //     d();
    //     (&d)();
    // }
}

#[test]
fn test_run() {
    {
        let mut list = vec![1, 2];
        let mut bm = || list.push(3);
        // print!("before call: {:?}", list);//编译错误， 因为在closure中是 mut reference捕获，所以不能再有 immut reference了
        bm();
        print!("after call: {:?}", list);
    }
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
        }

        let mut list = [Rectangle { width: 10 }, Rectangle { width: 15 }];

        let mut sort_info = vec!["".to_owned()];
        let value = String::from("value");
        // list.sort_by_key(|r|{ //编译不通过，因为value是 owner方式捕获， 这样它只实现FnOnce，而不能被多次调用
        //     sort_info.push(value);
        //     r.width
        // });
        println!("{:?}", list);
        let _ = sort_info;
        let _ = value;
    }
}
