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
mod async_in_trait {
    use std::future::Future;
    use std::pin::Pin;

    use futures::executor::block_on;
    use futures::future::BoxFuture;
    use futures::FutureExt;

//使用#[async_trait]是一个方法，这里不再举例，详细见 #[async_trait]自己的说明

    trait AsyncTrait {
        fn f1(&mut self) -> Pin<Box<dyn Future<Output=i32> + Send + '_>>;
    }

    impl AsyncTrait for i32 {
        fn f1(&mut self) -> Pin<Box<dyn Future<Output=i32> + Send + '_>> {
            async fn run(_self: &mut i32) -> i32 {
                println!("call run");
                *_self
            }//疑问？ run是async函数， 而self是引用，这里为什么没有 lifetime问题
            let fu = Box::pin(run(self));
            println!("after Box::pin");
            fu
        }
    }

    trait AsyncTrait2 {
        fn f1(&self) -> BoxFuture<'_, i32>;
    }

    impl AsyncTrait2 for i32 {
        fn f1(&self) -> BoxFuture<'_, i32> {
            let f = |_self: &i32| {
                let t = *_self;
                async move {
                    t
                }
            };
            f(self).boxed()
            // Box::pin(f(self)) //这个与上面的是一样的效果
        }
    }

    #[test]
    fn test_t1() {
        let mut data = 0i32;

        block_on(AsyncTrait::f1(&mut data));
        block_on(AsyncTrait2::f1(&data));
        println!("");
    }
}



