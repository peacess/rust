/// # copy_
/// 在Rust中Copy trait没有方法，
/// * copy动着是由编译器生成按位复制的，
/// * 虽然Copy一定要求Clone，但在copy发生时，并不会调用Clone中的方法（这是我以前理解错误的地方）
/// * 有Copy，那么要求clone方法与copy产生相同的效果（这是一个约束，并没有编译器要求），也就是这样实现：
/// fn clone(&self) -> Self{
///     *self
/// }
/// 所以大部都是#[derive(Copy, Clone)]，如果是其它形式，给出足够足够的理由
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

    let t = Point {
        x: 1,
        y: 2,
        // name:"name".to_owned()
    };
    let mut t2 = t;
    t2.x = 0;
    println!("{:?}", t);
}

/// Copy trait不是auto trait
#[test]
fn test_auto_copy() {
    #[derive(Debug)]
    struct D {
        a: i32,
    }

    let d = D { a: 10 };
    let d2 = d;
    let _ = d2;
    // println!("{:?}", d);
}
