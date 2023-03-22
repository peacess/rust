#[cfg(test)]
mod test {
    use std::sync::{Arc, Barrier};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn test_relaxed() {
        let a = Arc::new(AtomicBool::new(false));
        let b = Arc::new(AtomicBool::new(false));
        let store_a_b = |a_clone: Arc<AtomicBool>, b_clone: Arc<AtomicBool>, value: bool| {
            a_clone.store(value, Ordering::Relaxed);
            b_clone.store(value, Ordering::Relaxed);
        };
        let read_b_a = |a: Arc<AtomicBool>, b: Arc<AtomicBool>| {
            while !b.load(Ordering::Relaxed) {}
            let t = a.load(Ordering::Relaxed);
            if !t {
                println!("b is true, a is {}", t);
            }
        };
        for _ in 0..100000 {
            let a_clone = a.clone();
            let b_clone = b.clone();
            let t2 = thread::spawn(move || {
                read_b_a(a_clone, b_clone)
            });
            let a_clone = a.clone();
            let b_clone = b.clone();
            let t1 = thread::spawn(move || {
                store_a_b(a_clone, b_clone, true)
            });

            t1.join().expect("");
            t2.join().expect("");
            store_a_b(a.clone(), b.clone(), false);
        }
    }

    #[test]
    fn test_relaxed_wait() {
        let thread_num = num_cpus::get() - 2;
        let max_value = 10000;
        let a = Arc::new(AtomicUsize::new(0));
        let b = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(thread_num + 1));

        let mut threads = vec![];

        for _ in 0..thread_num {
            let a_clone = a.clone();
            let b_clone = b.clone();
            let barrier_clone = barrier.clone();
            let t = thread::spawn(move || {
                barrier_clone.wait();
                let mut v = 0;
                while v < max_value - 1 {
                    // fence(Ordering::SeqCst);
                    let b = b_clone.load(Ordering::Relaxed);
                    let a = a_clone.load(Ordering::Relaxed);

                    if b > a {
                        println!("b is {}, a is {}", b, a);
                    }
                    v = a;
                }
            });
            threads.push(t);
        }

        barrier.wait();
        for i in 0..max_value {
            // fence(Ordering::SeqCst);
            a.store(i, Ordering::Relaxed);
            b.store(i, Ordering::Relaxed);
        }

        for t in threads {
            let _ = t.join();
        }
    }

    /// [see](https://riptutorial.com/rust/example/21259/atomics-and-memory-ordering)
    #[cfg(any(test))]
    mod test_relaxed {
        use std::cell::UnsafeCell;
        use std::sync::{Arc, Barrier};
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::thread;

        struct UsizePair {
            atom: AtomicUsize,
            norm: UnsafeCell<usize>,
        }

        // UnsafeCell is not thread-safe. So manually mark our UsizePair to be Sync.
// (Effectively telling the compiler "I'll take care of it!")
        unsafe impl Sync for UsizePair {}

        impl UsizePair {
            pub fn new(v: usize) -> UsizePair {
                UsizePair {
                    atom: AtomicUsize::new(v),
                    norm: UnsafeCell::new(v),
                }
            }

            pub fn get(&self) -> (usize, usize) {
                let atom = self.atom.load(Ordering::Acquire); //Ordering::Acquire

                // If the above load operation is performed with `Acquire` ordering,
                // then all writes before the corresponding `Release` store is
                // guaranteed to be visible below.

                let norm = unsafe { *self.norm.get() };
                (atom, norm)
            }

            pub fn set(&self, v: usize) {
                unsafe { *self.norm.get() = v };

                // If the below store operation is performed with `Release` ordering,
                // then the write to `norm` above is guaranteed to be visible to all
                // threads that "loads `atom` with `Acquire` ordering and sees the same
                // value that was stored below". However, no guarantees are provided as
                // to when other readers will witness the below store, and consequently
                // the above write. On the other hand, there is also no guarantee that
                // these two values will be in sync for readers. Even if another thread
                // sees the same value that was stored below, it may actually see a
                // "later" value in `norm` than what was written above. That is, there
                // is no restriction on visibility into the future.

                self.atom.store(v, Ordering::Release); //Ordering::Release
            }
        }

        #[test]
        fn test_relaxed2() {
            let nthreads: usize = num_cpus::get() - 2;
            let niters: usize = 100000;
            let usize_pair = Arc::new(UsizePair::new(0));

            // Barrier is a counter-like synchronization structure (not to be confused
            // with a memory barrier). It blocks on a `wait` call until a fixed number
            // of `wait` calls are made from various threads (like waiting for all
            // players to get to the starting line before firing the starter pistol).
            let barrier = Arc::new(Barrier::new(nthreads + 1));

            let mut children = vec![];

            for _ in 0..nthreads {
                let upair = usize_pair.clone();
                let barrier = barrier.clone();
                children.push(thread::spawn(move || {
                    barrier.wait();

                    let mut v = 0;
                    while v < niters - 1 {
                        // Read both members `atom` and `norm`, and check whether `atom`
                        // contains a newer value than `norm`. See `UsizePair` impl for
                        // details.
                        let (atom, norm) = upair.get();
                        if atom > norm {
                            // If `Acquire`-`Release` ordering is used in `get` and
                            // `set`, then this statement will never be reached.
                            println!("Reordered! {} > {}", atom, norm);
                        }
                        v = atom;
                    }
                }));
            }

            barrier.wait();

            for v in 1..niters {
                usize_pair.set(v);
            }

            for child in children {
                let _ = child.join();
            }
        }
    }

    #[cfg(any(test))]
    mod test2 {
        /// [see](https://github.com/freepeace/code_styles/blob/master/atomic_volatile_order-cn.md)
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, fence, Ordering};
        use std::thread;

        #[test]
        fn test_atomic_volatile_order() {
            let a = Arc::new(AtomicBool::new(false));
            let b = Arc::new(AtomicBool::new(false));

            for _ in 0..100 {
                let a_clone = a.clone();
                let b_clone = b.clone();
                let t1 = thread::spawn(move || {
                    a_clone.store(true, Ordering::Relaxed);
                    fence(Ordering::Release);
                    b_clone.store(true, Ordering::Relaxed);
                });

                let a_clone = a.clone();
                let b_clone = b.clone();
                let t2 = thread::spawn(move || {
                    while !b_clone.load(Ordering::Relaxed) {}
                    fence(Ordering::Acquire);
                    let a = a_clone.load(Ordering::Relaxed);
                    println!("a = {},b = true", a);
                });

                let a_clone = a.clone();
                let b_clone = b.clone();
                let t3 = thread::spawn(move || {
                    while !a_clone.load(Ordering::Relaxed) {}
                    fence(Ordering::Acquire);
                    let b = b_clone.load(Ordering::Relaxed);
                    println!("a = true,b = {}", b);
                });

                let _ = t1.join();
                let _ = t2.join();
                let _ = t3.join();
            }
        }
    }
}