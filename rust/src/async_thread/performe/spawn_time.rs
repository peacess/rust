fn main() {
    // tokio::spawn: 1719 (ns/op)
    // std::thread::spawn: 13168 (ns/op)

    const MAX: u64 = 100;
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _t = rt.enter();
        {
            let start = std::time::Instant::now();
            for _i in 0..MAX {
                tokio::spawn(async {});
            }
            let du = start.elapsed();
            println!("tokio::spawn: {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        {
            let start = std::time::Instant::now();
            for _i in 0..MAX {
                tokio::spawn(async {});
            }
            let du = start.elapsed();
            println!("tokio::spawn: {} (ns/op)", du.as_nanos() / MAX as u128);
        }
        rt.shutdown_background();
    }
    {
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            std::thread::spawn(|| {});
        }
        let du = start.elapsed();
        println!("std::thread::spawn: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _task = smol::spawn(async {});
        }
        let du = start.elapsed();
        println!("smol::spawn: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
    {
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            let _task = smol::spawn(async {});
        }
        let du = start.elapsed();
        println!("smol::spawn: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
}
