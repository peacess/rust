fn main() {
    // std::sync::Mutex: 23 (ns/op)
    // std::sync::Mutex/CondVar: 156 (ns/op)
    // parking_lot::Mutex: 21 (ns/op)
    // parking_lot::Mutex/CondVar: 26 (ns/op)

    const MAX: u64 = 1_000_000;
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
}
