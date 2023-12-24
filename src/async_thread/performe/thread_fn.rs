
fn main() {
    const MAX: u64 = 1000_000_000;
    let mut t = std::thread::spawn(||{
        let start = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..MAX {
            let t = add(i);
            sum += t;
        }
        let du = start.elapsed();
        println!("sum: {} \n all: {}, t/s: {}", sum, du.as_nanos(),du.as_nanos()/MAX as u128);
    });
    let _ = t.join();
}

#[inline(never)]
fn add(c: u64) -> u64 {
    return c + 1;
}