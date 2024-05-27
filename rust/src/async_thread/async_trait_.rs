/// 使用struct来解决 trait不能实现 Send的问题
mod wrap_struct {
    use std::ops::Deref;

    trait T1 {
        fn f(&self);
    }

    struct ST1 {}

    impl T1 for ST1 {
        fn f(&self) {
            println!("impl f");
        }
    }

    struct AsyncTrait(Box<dyn T1>);
    // struct AsyncTrait(Box<dyn T1 + Send>); //直接指定是Send这样，就不需要为AsyncTrait实现Send trait了。
    // 因为一个struct如果所有字段都是Send那么，它自己也是Send

    impl Deref for AsyncTrait {
        type Target = Box<dyn T1>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    unsafe impl Send for AsyncTrait {}

    #[test]
    fn test_t1() {
        let t1 = ST1 {};
        let t = AsyncTrait(Box::new(t1));
        let handle = std::thread::spawn(move || {
            t.f();
        });
        handle.join().expect("");
    }
}

/// 使用struct来解决 trait不能实现 Send的问题
/// 在trait中的fn不能加async，这里给出替代的方法
/// [see](https://blog.theincredibleholk.org/blog/2022/04/18/how-async-functions-in-traits-could-work-in-rustc/)
#[cfg(test)]
mod async_in_trait {
    use std::future::Future;
    use std::pin::Pin;

    use futures::executor::block_on;
    use futures::future::BoxFuture;

    //使用#[async_trait]是一个方法，这里不再举例，详细见 #[async_trait]自己的说明

    trait AsyncTrait {
        fn f1<'a>(&mut self) -> Pin<Box<dyn Future<Output = i32> + Send + '_>>;
    }

    impl AsyncTrait for String {
        fn f1<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = i32> + Send + 'a>> {
            // async fn run<'a >(_self: &'a mut String) -> i32 {
            //     println!("call run {}", _self);
            //     0
            // }//疑问？ run是async函数， 而self是引用，这里为什么没有 lifetime问题
            //因为返回值中包含self的引用，那么函数的返回与self的lifetime是一样的，这个与返回struct中字段的引用是一样的
            //但是closure这样做时，需要给出明确的lifetime参数

            let run = |_self: &'a mut String| async move {
                println!("call run {}", _self);
                0
            };
            let fu = Box::pin(run(self));
            println!("after Box::pin");
            fu
        }
    }

    trait AsyncTrait2 {
        fn f1<'a>(&'a self) -> BoxFuture<'a, i32>;
    }

    impl AsyncTrait2 for i32 {
        fn f1<'a>(&'a self) -> BoxFuture<'a, i32> {
            let f = |_self: &'a i32| async move { *_self };
            // f(self).boxed()
            Box::pin(f(self)) //这个与上面的是一样的效果
        }
    }

    #[test]
    fn test_t1() {
        {
            let mut data = "hi".to_owned();
            block_on(data.f1());
        }

        {
            let mut data = 0i32;
            block_on(AsyncTrait2::f1(&data));
        }
        println!("");
    }
}
