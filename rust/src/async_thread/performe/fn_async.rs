fn main() {
    // call await: 9 (ns/op)
    // call fn: 4 (ns/op)

    const MAX: u64 = 1_000_000;
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let start = std::time::Instant::now();
            for i in 0..MAX {
                add_async(i).await;
            }
            let du = start.elapsed();
            println!("call await: {} (ns/op)", du.as_nanos() / MAX as u128);
        });
    }

    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let start = std::time::Instant::now();
            for i in 0..MAX {
                add(i);
            }
            let du = start.elapsed();
            println!("call fn: {} (ns/op)", du.as_nanos() / MAX as u128);
        });
    }
}

#[inline(never)]
async fn add_async(c: u64) {
    let _ = c + 1;
}

#[inline(never)]
fn add(c: u64) {
    let _ = c + 1;
}
