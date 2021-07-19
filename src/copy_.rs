#[test]
fn test_copy_dervive() {
    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
}

#[test]
fn test_copy_custom() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
        // name: String, //没有实现Copy编译不通过
    }

    impl Clone for Point {
        fn clone(&self) -> Self {
            *self
        }

        fn clone_from(&mut self, source: &Self) {
            self.x = source.x;
            self.y = source.y;
            // self.name = source.name.clone();
        }
    }
    impl Copy for Point {}

    let t = Point { x: 1, y: 2,
        // name:"name".to_owned()
    };
    let mut t2 = t;
    t2.x = 0;
    println!("{:?}", t);
}