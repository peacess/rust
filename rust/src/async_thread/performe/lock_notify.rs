use std::sync::Arc;

fn main() {
    // std::sync::Mutex: 23 (ns/op)
    // std::sync::Mutex/CondVar: 140 (ns/op)
    // parking_lot::Mutex: 21 (ns/op)
    // parking_lot::Mutex/CondVar: 25 (ns/op)
    // notify one -- wait: 3030 (ns/op)
    //
    const MAX: u64 = 100_000;
    {
        let mutex = std::sync::Mutex::new(0);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _lock = mutex.lock();
        }
        let du = start.elapsed();
        println!("std::sync::Mutex: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let mutex = std::sync::Mutex::new(0);
        let cond = std::sync::Condvar::new();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _lock = mutex.lock();
            cond.notify_one();
        }
        let du = start.elapsed();
        println!("std::sync::Mutex/CondVar: {} (ns/op)", du.as_nanos() / MAX as u128);
    }

    {
        let mutex = parking_lot::Mutex::new(0);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _lock = mutex.lock();
        }
        let du = start.elapsed();
        println!("parking_lot::Mutex: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let mutex = parking_lot::Mutex::new(0);
        let cond = parking_lot::Condvar::new();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _lock = mutex.lock();
            cond.notify_one();
        }
        let du = start.elapsed();
        println!("parking_lot::Mutex/CondVar: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let j = std::thread::spawn(|| {
            let mutex1 = Arc::new(parking_lot::Mutex::new(0));
            let cond1 = Arc::new(parking_lot::Condvar::new());
            let mutex1_clone = mutex1.clone();
            let cond1_clone = cond1.clone();

            let mutex2 = Arc::new(parking_lot::Mutex::new(0));
            let cond2 = Arc::new(parking_lot::Condvar::new());
            let mutex2_clone = mutex2.clone();
            let cond2_clone = cond2.clone();

            let j2 = std::thread::spawn(move || {
                for _i in 0..MAX {
                    {
                        let mut _lock = mutex1_clone.lock();
                        cond1_clone.wait(&mut _lock);
                    }
                    loop {
                        // avoid deadlock(if the notify_one is before wait, that will be deadlock)
                        let _lock = mutex2_clone.lock();
                        if cond2_clone.notify_one() {
                            break;
                        }
                    }
                }
            });

            std::thread::sleep(std::time::Duration::from_secs(1));
            let start = std::time::Instant::now();
            for _i in 0..MAX {
                loop {
                    // avoid deadlock(if the notify_one is before wait, that will be deadlock)
                    let _lock = mutex1.lock();
                    if cond1.notify_one() {
                        break;
                    }
                }
                {
                    let mut _lock = mutex2.lock();
                    cond2.wait(&mut _lock);
                }
            }
            let du = start.elapsed();
            println!("notify one -- wait: {} (ns/op)", du.as_nanos() / (MAX as u128 * 2));
            let _ = j2.join();
        });

        let _ = j.join();
    }
}
