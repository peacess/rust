use std::{
    marker::PhantomData,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
    thread::yield_now,
};

struct LocalValue<'a, T> {
    value: *const T,
    phantom: PhantomData<&'a T>,
}

unsafe impl<'a, T> Send for LocalValue<'a, T> {}

impl<'a, T> LocalValue<'a, T> {
    pub fn new(value: &T) -> Self {
        unsafe {
            Self {
                value: value as *const T,
                phantom: PhantomData,
            }
        }
    }
}

impl<'a, T> Deref for LocalValue<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

thread_local! {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
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

    let old = c.load(Ordering::Relaxed);
    assert_eq!(old, 1);
    c.store(2, Ordering::Relaxed);
    let new_value = c.load(Ordering::Relaxed);
    assert_eq!(new_value, 2);

    let joined = run.spawn(async move {
        let c2 = COUNTER.with(|counter| {
            assert_eq!(counter.load(Ordering::Relaxed), 1);
            println!("{:p}", counter);
            LocalValue::new(counter)
        });
        tokio::task::yield_now().await;
        async {}.await;
        let old = c2.load(Ordering::Relaxed);
        assert_eq!(old, 1);
        c2.store(3, Ordering::Relaxed);
        let new_value = c2.load(Ordering::Relaxed);
        assert_eq!(new_value, 3);
    });
    run.block_on(joined);
    let new_value = c.load(Ordering::Relaxed);
    assert_eq!(new_value, 2);
}
