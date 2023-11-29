use std::mem::size_of;

#[test]
fn size_of_() {
    const ALIGN: usize = 16;
    // (), empty enum/struct，[T; 0]  size of  == 0
    // bool == 1
    // Option<T> == size of T + size of Flag, sometimes the flag is zero
    // struct memory alignment
    {
        println!("{:ALIGN$}: {}", "()", size_of::<()>());
        println!("{:ALIGN$}: {}", "bool", size_of::<bool>());
        println!("{:ALIGN$}: {}", "char", size_of::<char>());
        println!("{:ALIGN$}: {}", "i8", size_of::<i8>());
        println!("{:ALIGN$}: {}", "i16", size_of::<i16>());
        println!("{:ALIGN$}: {}", "i32", size_of::<i32>());
        println!("{:ALIGN$}: {}", "i64", size_of::<i64>());
        println!("{:ALIGN$}: {}", "i128", size_of::<i128>());
        println!("{:ALIGN$}: {}", "usize", size_of::<usize>());
        println!("{:ALIGN$}: {}", "f32", size_of::<f32>());
        println!("{:ALIGN$}: {}", "f64", size_of::<f64>());
    }
    {
        println!("{:ALIGN$}: {}", "*T", size_of::<*const bool>());
        println!("{:ALIGN$}: {}", "[0]", size_of::<[u8; 0]>());
        println!("{:ALIGN$}: {}", "[1]", size_of::<[u8; 1]>());
        println!("{:ALIGN$}: {}", "slice", size_of::<&[u8]>());
        println!("{:ALIGN$}: {}", "&str", size_of::<&str>());
        println!("{:ALIGN$}: {}", "String", size_of::<String>());
    }
    {
        enum EmptyEnum {}
        println!("{:ALIGN$}: {}", "enum Empty", size_of::<EmptyEnum>());
        struct EmptyStruct {}
        println!("{:ALIGN$}: {}", "struct empty", size_of::<EmptyStruct>());
        struct EmptyTuple();
        println!("{:ALIGN$}: {}", "tuple empty", size_of::<EmptyTuple>());
        //有flag, 数据不同时flag的类型不一样
        println!("{:ALIGN$}: {}", "option<u8>", size_of::<Option<u8>>());
        println!("{:ALIGN$}: {}", "option<u64>", size_of::<Option<u64>>());
        //没有flag,
        println!("{:ALIGN$}: {}", "option<String>", size_of::<Option<String>>());
    }
}