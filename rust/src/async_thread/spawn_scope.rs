#[cfg(test)]
mod test {
    use std::time::Duration;

    /// std::thread::scope 方法会是同步的，会等其中的spawn运行完成
    /// scope中方法在返回之前会等所有的线程完成，也就是会在drop时join线程。
    /// scope中不支持 async方法
    /// scope中不支持 async方法
    #[test]
    fn test_scope() {
        println!("before start");
        std::thread::scope(|s| {
            s.spawn(|| {
                std::thread::sleep(Duration::from_secs(1));
                println!("first");
            });
            s.spawn(|| {
                println!("two");
            });
        });
        println!("after start");
    }
}
