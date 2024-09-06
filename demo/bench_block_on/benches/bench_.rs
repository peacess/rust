use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use criterion::{criterion_group, criterion_main, Criterion};

struct SampleFuture(i32);

impl Future for SampleFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 0 {
            Poll::Ready(())
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// see [block on](https://github.com/async-rs/async-task/blob/master/examples/block.rs)
fn block_on<F: Future>(future: F) -> F::Output {
    use std::{cell::RefCell, task::Waker};

    use crossbeam::sync::Parker;

    let mut future = std::pin::pin!(future);

    thread_local! {
        static CACHE: RefCell<(Parker, Waker)> = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = waker_fn::waker_fn(move || unparker.unpark());
            RefCell::new((parker, waker))
        };
    }

    CACHE.with(|cache| {
        let (parker, waker) = &mut *cache.try_borrow_mut().ok().expect("recursive `block_on`");

        let cx = &mut Context::from_waker(&waker);
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

    c.bench_function("directly code", |b| {
        b.iter(|| {
            block_on(SampleFuture(TIMES));
        })
    });
    c.bench_function("futures_lite", |b| {
        b.iter(|| {
            futures_lite::future::block_on(SampleFuture(TIMES));
        })
    });
    c.bench_function("futures", |b| {
        b.iter(|| {
            futures::executor::block_on(SampleFuture(TIMES));
        })
    });
    c.bench_function("smol", |b| {
        b.iter(|| {
            smol::block_on(SampleFuture(TIMES));
        })
    });

    let tokio_ = tokio::runtime::Runtime::new().expect("");
    c.bench_function("tokio", |b| {
        b.iter(|| {
            tokio_.block_on(SampleFuture(TIMES));
        })
    });
    c.bench_function("extreme", |b| {
        b.iter(|| {
            extreme::run(SampleFuture(TIMES));
        })
    });
    c.bench_function("async_std", |b| {
        b.iter(|| {
            async_std::task::block_on(SampleFuture(TIMES));
        })
    });
    c.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
