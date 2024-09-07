fn main() {
    // std::sync::mpsc::sync_channel: 33 (ns/op)
    // std::sync::mpsc::channel: 40 (ns/op)
    // kanal::bounded: 29 (ns/op)
    // kanal::unbounded: 29 (ns/op)
    // kanal::bounded_async: 70 (ns/op)
    // kanal::unbounded_async: 73 (ns/op)
    // crossbeam_channel::bounded: 55 (ns/op)
    // crossbeam_channel::unbounded: 55 (ns/op)
    // tokio::sync::mpsc::channel: 229 (ns/op)
    // tokio::sync::mpsc::unbounded_channel: 59 (ns/op)
    // tokio::sync::watch::channel: 294 (ns/op)
    // tokio::sync::broadcast::channel: 140 (ns/op)

    const MAX: u64 = 1_000_000;
    {
        let (s, _r) = std::sync::mpsc::sync_channel(MAX as usize);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("std::sync::mpsc::sync_channel: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let (s, _r) = std::sync::mpsc::channel();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("std::sync::mpsc::channel: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let (s, _r) = kanal::bounded(MAX as usize);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("kanal::bounded: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let (s, _r) = kanal::unbounded();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("kanal::unbounded: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (s, _r) = kanal::bounded_async(MAX as usize);
            let start = std::time::Instant::now();
            for _i in 0..MAX {
                let _ = s.send(0).await;
            }
            let du = start.elapsed();
            println!("kanal::bounded_async: {} (ns/op)", du.as_nanos() / MAX as u128);
        });
    }
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (s, _r) = kanal::unbounded_async();
            let start = std::time::Instant::now();
            for _i in 0..MAX {
                let _ = s.send(0).await;
            }
            let du = start.elapsed();
            println!("kanal::unbounded_async: {} (ns/op)", du.as_nanos() / MAX as u128);
        });
    }
    {
        let (s, _r) = crossbeam_channel::bounded(MAX as usize);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("crossbeam_channel::bounded: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let (s, _r) = crossbeam_channel::unbounded();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("crossbeam_channel::unbounded: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (s, _r) = tokio::sync::mpsc::channel(MAX as usize);
            let start = std::time::Instant::now();
            for _i in 0..MAX {
                let _ = s.send(0).await;
            }
            let du = start.elapsed();
            println!("tokio::sync::mpsc::channel: {} (ns/op)", du.as_nanos() / MAX as u128);
        });
    }
    {
        let (s, _r) = tokio::sync::mpsc::unbounded_channel();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("tokio::sync::mpsc::unbounded_channel: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let (s, _r) = tokio::sync::watch::channel(MAX as usize);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("tokio::sync::watch::channel: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let (s, _r) = tokio::sync::broadcast::channel(MAX as usize);
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _ = s.send(0);
        }
        let du = start.elapsed();
        println!("tokio::sync::broadcast::channel: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
}
