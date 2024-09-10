fn main() {
    // dashmap::DashMap:insert 781 (ns/op)
    // dashmap::DashMap:get 483 (ns/op)
    // dashmap::DashMap:get_mut 473 (ns/op)
    // std::collections::HashMap:insert 622 (ns/op)
    // std::collections::HashMap:get 404 (ns/op)
    // std::collections::HashMap:get_mut 409 (ns/op)
    // std::collections::BTreeMap:insert 963 (ns/op)
    // std::collections::BTreeMap:get 581 (ns/op)
    // std::collections::BTreeMap:get_mut 576 (ns/op)

    const MAX: u64 = 1_000;
    {
        let map = dashmap::DashMap::new();
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.insert(key, key);
            }
            let du = start.elapsed();
            println!("dashmap::DashMap:insert {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.get(&key);
            }
            let du = start.elapsed();
            println!("dashmap::DashMap:get {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.get_mut(&key);
            }
            let du = start.elapsed();
            println!("dashmap::DashMap:get_mut {} (ns/op)", du.as_nanos() / MAX as u128);
        }
    }
    {
        let mut map = std::collections::HashMap::new();
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.insert(key, key);
            }
            let du = start.elapsed();
            println!("std::collections::HashMap:insert {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.get(&key);
            }
            let du = start.elapsed();
            println!("std::collections::HashMap:get {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.get_mut(&key);
            }
            let du = start.elapsed();
            println!("std::collections::HashMap:get_mut {} (ns/op)", du.as_nanos() / MAX as u128);
        }
    }

    {
        let mut map = std::collections::BTreeMap::new();
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.insert(key, key);
            }
            let du = start.elapsed();
            println!("std::collections::BTreeMap:insert {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.get(&key);
            }
            let du = start.elapsed();
            println!("std::collections::BTreeMap:get {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for key in 0..MAX {
                map.get_mut(&key);
            }
            let du = start.elapsed();
            println!("std::collections::BTreeMap:get_mut {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let mut ids_u32 = Vec::with_capacity(MAX as usize);
            {
                for key in 0..MAX {
                    ids_u32.push(key);
                }
                ids_u32.sort();
            }
            let start = std::time::Instant::now();
            for key in &ids_u32 {
                let _ = ids_u32.binary_search(&key);
            }
            let du = start.elapsed();
            println!("sorted vec get:  {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let a_data = std::sync::atomic::AtomicU64::new(0);
            let start = std::time::Instant::now();
            for key in 0..MAX {
                a_data.store(key, std::sync::atomic::Ordering::Relaxed);
            }
            let du = start.elapsed();
            println!("std::sync::atomic::AtomicU64:  {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let mut a_data = 0u64;
            let start = std::time::Instant::now();
            for key in 0..MAX {
                a_data = key;
            }
            let du = start.elapsed();
            println!("u64:  {} (ns/op)", du.as_nanos() / MAX as u128);
        }
    }
}
