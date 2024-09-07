fn main() {
    const MAX: u64 = 1_000_000;
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _t = rt.enter();
        let start = std::time::Instant::now();
        for _i in 0..MAX {
            tokio::spawn(async {});
        }
        let du = start.elapsed();
        // tokio::spawn: 2731567794, ns/op: 2731
        println!("tokio::spawn: {}, ns/op: {}", du.as_nanos(), du.as_nanos() / MAX as u128);
    }
    {}
}
