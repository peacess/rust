fn main() {
    const MAX: u64 = 1_000_000;
    let t = std::thread::spawn(|| {
        let start = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..MAX {
            let t = add(i);
            sum += t;
        }
        let du = start.elapsed();
        // sum: 500000500000
        // all: 14888836, ns/op: 14
        println!("sum: {} \n all: {}, ns/op: {}", sum, du.as_nanos(), du.as_nanos() / MAX as u128);
    });
    let _ = t.join();
}

#[inline(never)]
fn add(c: u64) -> u64 {
    c + 1
}
