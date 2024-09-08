fn main() {
    const MAX: u64 = 100_000_000;
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
    }
}
