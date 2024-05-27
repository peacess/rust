fn main() {
    const MAX: u64 = 1000_000_000;
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let start = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..MAX {
            let t = add(i).await;
            sum += t;
        }
        let du = start.elapsed();
        println!("sum: {} \n all: {}, t/s: {}", sum, du.as_nanos(), du.as_nanos() / MAX as u128);
    });
}

#[inline(never)]
async fn add(c: u64) -> u64 {
    return c + 1;
}
