#![allow(unused_variables)]

#[cfg(test)]
mod test {
    #[test]
    fn test_format() {
        let debug = format!("{:?}", format_args!("{} foo {:?}", 1, 2));
        let display = format!("{}", format_args!("{} foo {:?}", 1, 2));
        assert_eq!("1 foo 2", display);
        assert_eq!(display, debug);
        //只把format_args!用在fmt相关函数的参数上面

        {
            let t = format_args!("{} foo {:?}", 1, 2);
            println!("{}", 1);
        }
    }
}
