use std::cell::{OnceCell, RefCell};

fn main() {
    affinity();
    // channel time rust/src/async_thread/performe/channel_time.rs
    // map and vec  rust/src/async_thread/performe/data.rs
}

fn affinity() {
    thread_local! {
        static CU_CORE: RefCell<core_affinity::CoreId> = const { RefCell::new(core_affinity::CoreId { id: usize::MAX }) };
    }
    static mut VEC_CORE: OnceCell<Vec<core_affinity::CoreId>> = OnceCell::new();
    unsafe {
        VEC_CORE.get_or_init(|| core_affinity::get_core_ids().unwrap());
    }
    println!("current thread id {:?}", std::thread::current().id());
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(1)
        .on_thread_start(|| {
            println!("on_thread_start thread start {:?}", std::thread::current().id());
            unsafe {
                if let Some(core) = VEC_CORE.get_mut().unwrap().pop() {
                    core_affinity::set_for_current(core);
                    CU_CORE.with_borrow_mut(|c| {
                        println!("CU_CORE {:?}", core);
                        *c = core;
                    });
                }
            }
        })
        .on_thread_stop(|| {
            println!("on_thread_stop thread stop {:?}", std::thread::current().id());
            CU_CORE.with_borrow_mut(|c| {
                if c.id != usize::MAX {
                    unsafe {
                        VEC_CORE.get_mut().unwrap().push(*c);
                    }
                    *c = core_affinity::CoreId { id: usize::MAX };
                }
            });
        })
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        println!("block_on thread id {:?}", std::thread::current().id());
        {
            tokio::spawn(async move {
                println!("tokio spawn {:?}", std::thread::current().id());
            })
            .await
            .unwrap();

            tokio::task::spawn_blocking(move || {
                println!("spawn_blocking {:?}", std::thread::current().id());
                tokio::runtime::Handle::current().block_on(async {
                    println!("spawn blocking  Handle::current().block_on: {:?}", std::thread::current().id());
                    tokio::spawn(async move {
                        println!("spawn blocking  tokio spawn {:?}", std::thread::current().id());
                    })
                    .await
                    .unwrap();
                })
            })
            .await
            .unwrap();
        }
        {
            let local = tokio::task::LocalSet::new();
            local
                .run_until(async {
                    println!("runtime local.block_on {:?}", std::thread::current().id());
                    tokio::task::spawn_local(async {
                        println!("runtime spawn_local {:?}", std::thread::current().id());

                        tokio::task::spawn_local(async {
                            println!("runtime spawn_local inner {:?}", std::thread::current().id());
                        })
                        .await
                        .unwrap();
                    })
                    .await
                    .unwrap();
                })
                .await;
        }
    });

    {
        let local = tokio::task::LocalSet::new();
        local.block_on(&runtime, async move {
            println!("local.block_on {:?}", std::thread::current().id());
            let join_handle = tokio::task::spawn_local(async {
                println!("spawn_local {:?}", std::thread::current().id());

                let join_handle = tokio::task::spawn_local(async {
                    println!("spawn_local inner {:?}", std::thread::current().id());
                });
                join_handle.await.unwrap();
            });
            join_handle.await.unwrap();
        });
    }

    runtime.shutdown_timeout(std::time::Duration::from_secs(1));
}
