use std::{
    cell::Cell,
    marker::PhantomData,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
    thread::yield_now,
};

struct LocalValue<'a, T> {
    value: *const T,
    phantom: PhantomData<&'a T>,
}

unsafe impl<T> Send for LocalValue<'_, T> {}

impl<T> LocalValue<'_, T> {
    pub fn new(value: &T) -> Self {
        unsafe {
            Self {
                value: value as *const T,
                phantom: PhantomData,
            }
        }
    }
}

impl<T> Deref for LocalValue<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

thread_local! {
    static COUNTER: Cell<i32> = const { Cell::new(1) };
}

#[test]
fn test_thread_local_value() {
    let run = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    run.enter();
    let c = COUNTER.with(|counter| {
        assert_eq!(counter.load(Ordering::Relaxed), 1);
        println!("{:p}", counter);
        LocalValue::new(counter)
    });

    let old = c.get();
    assert_eq!(old, 1);
    c.set(2);
    let new_value = c.get();
    assert_eq!(new_value, 2);

    let joined = run.spawn(async move {
        let c2 = COUNTER.with(|counter| {
            assert_eq!(counter.load(Ordering::Relaxed), 1);
            println!("{:p}", counter);
            LocalValue::new(counter)
        });
        tokio::task::yield_now().await;
        async {}.await;
        let old = c2.get();
        assert_eq!(old, 1);
        c2.set(3);
        let new_value = c2.get();
        assert_eq!(new_value, 3);
    });
    run.block_on(joined);
    let new_value = c.get();
    assert_eq!(new_value, 2);
    run.shutdown_background();
}

thread_local! {
    static COUNTER_ASYNC: Cell<i32> = Cell::new(1);
}

#[test]
fn test_thread_local_async() {
    let run = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    run.enter();

    let re = run.spawn(async move {
        COUNTER_ASYNC.with(|counter| {
            assert_eq!(counter.get(), 1);
            // let _ = async {  }.await;
            counter.set(2);
        })
    });
    run.block_on(re);
    COUNTER_ASYNC.with(|counter| {
        assert_eq!(counter.get(), 1);
    });

    run.shutdown_background();
}
