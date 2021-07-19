#[test]
fn test_closure() {
    /// 总结
    /// 1. FnOnce只能调用一次
    /// 2. 加move的closure可以是任意三种
    /// 3. 加move的

    fn fn_once<F>(func: F) where F: FnOnce() {
        func();//只能调用一次，&func也不行
    }
    {
        let mut a = "fn_once".to_owned();
        let f = || {
            // a = capture(a);
            println!("{}", a)
        };
        let f2 = f;
        fn_once(f);//f 自动实现了 Copy,所以可以再一次使用变量f
        fn_once(f2);
    }
    {
        fn fn_once<F>(func: F) where F: FnOnce() { func() }
        let a = "fn_once move".to_owned();
        let f = move || println!("{}", a);
        f();
        (&f)();
        fn_once(&f);//可以多次调用
        fn_once(f);
        //fn_once(f); //error[E0382]: use of moved value: `f`。有move时不会实现 Copy
    }
    fn fn_<F>(func: F) where F: Fn() {
        func();
    }
    {
        let a = "fn".to_owned();
        let f = || println!("{}", a);
        let f2 = f;
        fn_(f);//f 自动实现了 Copy,所以可以再一次使用变量f
        fn_(f2);
    }
    {
        let a = "fn move".to_owned();
        let f = move || println!("{}", a);
        fn_(&f);
        fn_(f);
    }

    fn fn_mut<F>(mut func: F) where F: FnMut() {
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

