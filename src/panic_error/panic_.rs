#[cfg(test)]
mod test {
    use std::panic;

    /// unwind vs abort panic https://users.rust-lang.org/t/panic-unwind-vs-abort/9928
    /// set_hook vs catch_unwind
    #[test]
    fn test_panic_unwind() {
        let result = panic::catch_unwind(|| {
            println!("in catch_unwind");
        });

        let t: Option<i32> = None;
        t.unwrap();

        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }

    #[test]
    fn test_panic() {
        let result = panic::catch_unwind(|| {
            println!("hello!");
        });
        assert!(result.is_ok());

        let result = panic::catch_unwind(|| {
            panic!("oh no!");
        });
        assert!(result.is_err());
    }
}