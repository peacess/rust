#[cfg(test)]
mod test {
    struct Data<T> {
        p: *mut T,
    }

    impl<T> Data<T> {
        fn new(t: T) -> Self {
            Data {
                p: Box::into_raw(Box::new(t)),
            }
        }
    }

    impl<T> Drop for Data<T> {
        fn drop(&mut self) {
            if !self.p.is_null() {
                unsafe {
                    let _ = Box::from_raw(self.p);
                }
                self.p = std::ptr::null_mut();
            }
        }
    }

    #[test]
    fn test_drop() {
        let mut x = 40;
        let d = Data::new(&x);
        drop(d);
        let b = Box::new(&mut x);
        // println!("{}", x);
        let _ = b;
    }
}