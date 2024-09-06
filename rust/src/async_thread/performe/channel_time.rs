fn main() {
    const MAX: u64 = 1_000_000;
    {
        let (s, r) = std::sync::mpsc::sync_channel(MAX as usize);
        let start = std::time::Instant::now();
        for i in 0..MAX {
            s.send(0);
        }
        let du = start.elapsed();
        // std::sync::mpsc::sync_channel: 65689693, ns/op: 65
        println!("std::sync::mpsc::sync_channel: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
    }
    {
        let (s, r) = std::sync::mpsc::channel();
        let start = std::time::Instant::now();
        for i in 0..MAX {
            s.send(0);
        }
        let du = start.elapsed();
        // std::sync::mpsc::channel: 80580252, ns/op: 80
        println!("std::sync::mpsc::channel: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
    }
    {
        let (s, r) = kanal::bounded(MAX as usize);
        let start = std::time::Instant::now();
        for i in 0..MAX {
            s.send(0);
        }
        let du = start.elapsed();
        // kanal::bounded: 58510467, ns/op: 58
        println!("kanal::bounded: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
    }
    {
        let (s, r) = kanal::unbounded();
        let start = std::time::Instant::now();
        for i in 0..MAX {
            s.send(0);
        }
        let du = start.elapsed();
        // kanal::unbounded: 59676999, ns/op: 59
        println!("kanal::unbounded: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
    }
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (s, r) = kanal::bounded_async(MAX as usize);
            let start = std::time::Instant::now();
            for i in 0..MAX {
                let _ = s.send(0).await;
            }
            let du = start.elapsed();
            // kanal::bounded_async: 140576046, ns/op: 140
            println!("kanal::bounded_async: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
        });
    }
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (s, r) = kanal::unbounded_async();
            let start = std::time::Instant::now();
            for i in 0..MAX {
                let _ = s.send(0).await;
            }
            let du = start.elapsed();
            // kanal::unbounded_async: 144269085, ns/op: 144
            println!("kanal::unbounded_async: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
        });
    }
}
