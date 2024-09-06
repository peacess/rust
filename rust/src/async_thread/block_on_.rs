use std::{
    future::Future,
    task::{Context, Poll, Waker},
};

use criterion::{criterion_group, criterion_main, Criterion};
use crossbeam::sync::Parker;
use waker_fn::waker_fn;

/// 比较 block_on的多种实现版，最后再自己实现一个
///

fn block_on_my<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);
    // let future: std::pin::Pin<&mut F> = future;
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn::waker_fn(move || unparker.unpark());
    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
    }
}

/// Runs a future to completion on the current thread.
/// [See](https://github.com/cambricorp/byo-block-on/blob/master/examples/v2.rs)
fn block_on_custom_normal<F: Future>(future: F) -> F::Output {
    // Pin the future on the stack.
    pin_utils::pin_mut!(future);

    // Parker and waker associated with the current thread.
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());

    // Create the task context.
    let cx = &mut Context::from_waker(&waker);

    // Keep polling the future until completion.
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
    }
}

/// Runs a future to completion on the current thread.
/// [See](https://github.com/cambricorp/byo-block-on/blob/master/examples/v4.rs)
fn block_on_custom_cache<F: Future>(future: F) -> F::Output {
    // Pin the future on the stack.
    let mut future = std::pin::pin!(future);

    thread_local! {
        // Parker and waker associated with the current thread.
        static CACHE: std::cell::RefCell<(Parker, Waker)> = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = waker_fn(move || unparker.unpark());
            std::cell::RefCell::new((parker, waker))
        };
    }

    CACHE.with(|cache| {
        // Panic if `block_on()` is called recursively.
        let (parker, waker) = &mut *cache.try_borrow_mut().expect("recursive `block_on`");

        // Create the task context.
        let cx = &mut Context::from_waker(waker);

        // Keep polling the future until completion.
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("compare: ");
    const TIMES: i32 = 2;
    let tokio_ = tokio::runtime::Runtime::new().expect("");
    let _ = &tokio_;
    c.bench_function("block_on_async_std", |b| {
        b.iter(|| {
            async_std::task::block_on(async { 1 });
        })
    });
    c.bench_function("block_on_futures", |b| {
        b.iter(|| {
            futures::executor::block_on(async { 1 });
        })
    });
    let r = tokio::runtime::Builder::new_current_thread().build().unwrap();
    c.bench_function("block_on_tokio", |b| {
        b.iter(|| {
            r.block_on(async { 1 });
        })
    });
    c.bench_function("block_on_smol", |b| {
        b.iter(|| {
            smol::block_on(async { 1 });
        })
    });
    c.bench_function("block_on_futures_lite", |b| {
        b.iter(|| {
            futures_lite::future::block_on(async { 1 });
        })
    });
    c.bench_function("block_on_extreme", |b| {
        b.iter(|| {
            extreme::run(async { 1 });
        })
    });
    c.bench_function("block_on_custom_normal_test", |b| {
        b.iter(|| {
            block_on_custom_normal(async { 1 });
        })
    });
    c.bench_function("block_on_custom_cache_test", |b| {
        b.iter(|| {
            block_on_custom_cache(async { 1 });
        })
    });

    c.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
