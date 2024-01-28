#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::time::Duration;

    use dashmap::DashMap;

    // DashMap get/get_mut similar  WRLock
    // cargo test -- --nocapture
    const SECONDS: u64 = 1;

    #[test]
    fn get_mut_mut() {
// 一个线程持有get_mut, 别一个线程是否可以拿到get_mut,它们的先后关系是什么
        let map = Arc::new(DashMap::new());
        let key = "test_id_".to_string();
        map.insert(key.clone(), "value".to_string());
        let key_thread1 = key.clone();
        let map_thread1 = map.clone();
        println!("get_mut_mut--------------- ");
        let thread1 = std::thread::spawn(move || {
            let v = map_thread1.get_mut(&key_thread1);
            println!("get mut: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get mut: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let key_thread2 = key.clone();
        let map_thread2 = map.clone();
        let thread2 = std::thread::spawn(move || {
            let v = map_thread2.get_mut(&key_thread2);
            println!("get mut: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get mut: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let _ = thread1.join();
        let _ = thread2.join();
        println!("--------------- get_mut_mut");
    }

    #[test]
    fn get_get() {
// 一个线程持有get, 别一个线程是否可以拿到get,它们的先后关系是什么
        let map = Arc::new(DashMap::new());
        let key = "test_id_".to_string();
        map.insert(key.clone(), "value".to_string());
        let key_thread1 = key.clone();
        let map_thread1 = map.clone();
        println!("get_get--------------- ");
        let thread1 = std::thread::spawn(move || {
            let v = map_thread1.get(&key_thread1);
            println!("get: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let key_thread2 = key.clone();
        let map_thread2 = map.clone();
        let thread2 = std::thread::spawn(move || {
            let v = map_thread2.get(&key_thread2);
            println!("get: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let _ = thread1.join();
        let _ = thread2.join();
        println!("--------------- get_get");
    }

    #[test]
    fn get_mut_get() {
// 一个线程持有get_mut, 别一个线程是否可以拿到get,它们的先后关系是什么
        let map = Arc::new(DashMap::new());
        let key = "test_id_".to_string();
        map.insert(key.clone(), "value".to_string());
        let key_thread1 = key.clone();
        let map_thread1 = map.clone();
        println!("get_mut_get--------------- ");
        let thread1 = std::thread::spawn(move || {
            let v = map_thread1.get_mut(&key_thread1);
            println!("get mut: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get mut: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let key_thread2 = key.clone();
        let map_thread2 = map.clone();
        let thread2 = std::thread::spawn(move || {
            let v = map_thread2.get(&key_thread2);
            println!("get: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let _ = thread1.join();
        let _ = thread2.join();
        println!("--------------- get_mut_get");
    }

    #[test]
    fn get_get_mut() {
// 一个线程持有get, 别一个线程是否可以拿到get_mut,它们的先后关系是什么
        let map = Arc::new(DashMap::new());
        let key = "test_id_".to_string();
        map.insert(key.clone(), "value".to_string());
        let key_thread1 = key.clone();
        let map_thread1 = map.clone();
        println!("get_get_mut--------------- ");
        let thread1 = std::thread::spawn(move || {
            let v = map_thread1.get(&key_thread1);
            println!("get: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get: thread1");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let key_thread2 = key.clone();
        let map_thread2 = map.clone();
        let thread2 = std::thread::spawn(move || {
            let v = map_thread2.get_mut(&key_thread2);
            println!("get mut: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
            drop(v);
            println!("drop get mut: thread2");
            std::thread::sleep(Duration::from_secs(SECONDS));
        });

        let _ = thread1.join();
        let _ = thread2.join();
        println!("--------------- get_get_mut");
    }
}