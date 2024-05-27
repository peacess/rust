// #![feature(raw)]

#[cfg(test)]
mod test {
    struct A<'a> {
        name: &'a str,
    }

    #[test]
    fn test() {
        let mut a = A { name: "I am here" };
        let t: *mut A = &mut a as *mut A;
        println!("{:p}", t);
        // std::raw::
    }
}
