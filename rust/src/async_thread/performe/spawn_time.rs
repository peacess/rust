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
        // tokio::spawn: 1026 (ns/op)
        println!("tokio::spawn: {} (ns/op)", du.as_nanos() / MAX as u128);
    }
}
